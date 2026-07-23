use crate::config::RunnerConfig;
use crate::docker::DockerManager;
use crate::job::Job;
use crate::job::error::{JobExecutionError, JobMetadataError};
use crate::job::metadata::JobMetadata;

use std::path::{Path, PathBuf};

use broker_client::BrokerClient;
use socket_protocol::messages::config_runner::ConfigRunner;
use socket_protocol::messages::job_error::JobError;
use socket_protocol::messages::job_finished::JobFinished;
use socket_protocol::messages::job_started::JobStarted;
use socket_protocol::messages::new_job::{NewJob, NewJobAck, NewJobNack};
use socket_protocol::{SocketProtocolClient, SocketProtocolError};
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tracing_subscriber;

mod api {
    #[cfg(feature = "api-1-54")]
    #[allow(unused_imports)]
    pub use api_1_54::*;

    #[cfg(feature = "api-1-50")]
    #[allow(unused_imports)]
    pub use api_1_50::*;

    #[cfg(feature = "api-1-54")]
    pub const API_VERSION: &str = "1.54";
    #[cfg(feature = "api-1-50")]
    pub const API_VERSION: &str = "1.50";
}

mod config;
mod docker;
mod hardware;
mod job;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("=== Hilltop Runner ===");

    // Load runner configuration
    let config_dir = PathBuf::from("./config");
    let runner_config_path = config_dir.join("runner.json");

    tracing::info!(
        "Loading runner config from {}...",
        runner_config_path.display()
    );
    let config_content = std::fs::read_to_string(&runner_config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read runner.json: {:?}", e))?;
    let runner_config = RunnerConfig::from_json(&config_content)?;

    tracing::info!(
        "Found {} images and {} hardware configurations",
        runner_config.available_images.len(),
        runner_config.hardware_configurations.len()
    );

    tracing::info!("Available Images:");
    for img in &runner_config.available_images {
        tracing::info!("  - {} (tag: {})", img.name, img.tag);
    }

    tracing::info!("Available Hardware:");
    for hw in &runner_config.hardware_configurations {
        tracing::info!("  - {}", hw.config_name);
    }

    let runner_slug = runner_config.runner_slug.clone();
    let api_key =
        std::fs::read_to_string(config_dir.join(runner_config.runner_api_key_path.clone()))
            .map_err(|e| anyhow::anyhow!("Failed to read API key: {:?}", e))?
            .trim()
            .to_string();

    tracing::info!("Loaded broker configuration:");
    tracing::info!(" - Runner Slug: {}", runner_slug);
    tracing::info!(" - API Key: [Redacted]");
    tracing::info!(" - Broker URL: {}", runner_config.broker_url);
    tracing::info!(" - Broker Socket URL: {}", runner_config.broker_socket_url);

    // Connect to broker

    let mut broker_client = BrokerClient::new(runner_config.broker_url.clone())
        .await
        .map_err(|err| anyhow::anyhow!("Failed to create broker client: {err:?}"))?;

    broker_client
        .login(runner_slug.clone(), api_key.clone())
        .await
        .map_err(|err| anyhow::anyhow!("Failed to login to broker: {err:?}"))?;

    tracing::info!("Successfully logged in to broker API");

    let mut socket_client = SocketProtocolClient::connect(&runner_config.broker_socket_url)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to connect to broker socket: {err:?}"))?;

    socket_client
        .hello()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to send HELLO_RUNNER: {err:?}"))?;

    socket_client
        .config(ConfigRunner::new(
            runner_config.runner_slug.clone(),
            api_key.clone(),
        ))
        .await
        .map_err(|err| anyhow::anyhow!("Failed to send CONFIG_RUNNER: {err:?}"))?;

    tracing::info!("Successfully logged in to broker socket protocol server");

    // Prepare docker

    let mut docker_manager = DockerManager::new(config_dir.clone(), runner_config.clone()).await?;

    tracing::info!("=== Building Images ===");
    for image_config in runner_config.available_images.iter() {
        docker_manager
            .build_image(
                &image_config.name,
                &image_config.tag,
                &Path::new(&image_config.image),
            )
            .await?;
    }

    // Start executor loop

    tracing::info!("=== Starting Executor Loop ===");
    executor_loop(docker_manager, broker_client, socket_client, runner_config).await?;

    Ok(())
}

pub type JobResult = Result<Job, (String, JobExecutionError)>;

/// Execute the runner loop. This consists of two components:
/// 1. Communication Node - handles runner <-> broker comms
/// 2. Execution Node - handles container orchestration
async fn executor_loop(
    mut docker: DockerManager,
    broker_client: BrokerClient,
    mut socket_client: SocketProtocolClient,
    runner_config: RunnerConfig,
) -> anyhow::Result<()> {
    let (to_exec, mut from_comms) = mpsc::channel::<CommNodeMessage>(100);
    let (to_comms, mut from_exec) = mpsc::channel::<ExecNodeMessage>(100);

    let comms_node = tokio::spawn(async move {
        loop {
            tokio::select! {
                job_request_result = socket_client.wait_new_job() => {
                    if let Err(e) = handle_next_job(job_request_result, &broker_client, &mut socket_client, &to_exec, &runner_config).await {
                        tracing::error!("Error handling job request: {e:?}");
                        break e;
                    }
                }
                msg_result = from_exec.recv() => {
                    if let Err(e) = handle_msg_from_exec_node(msg_result, &broker_client, &mut socket_client).await {
                        tracing::error!("Error handling message from exec node: {e:?}");
                        break e;
                    }
                }
            }
        }
    });
    let exec_node = tokio::spawn(async move {
        let mut pending_jobs = vec![];
        let mut running_jobs = JoinSet::new();

        loop {
            tokio::select! {
                comm_msg_result = from_comms.recv() => {
                    if let Err(e) = handle_msg_from_comms_node(comm_msg_result, &mut docker, &to_comms, &mut pending_jobs, &mut running_jobs).await {
                        tracing::error!("Error handling message from comms node: {e:?}");
                        break e;
                    }
                }
                Some(res) = running_jobs.join_next() => {
                    match res {
                        Ok(inner_res) => {
                            if let Err(e) = handle_completed_job(inner_res, &mut docker, &to_comms, &mut pending_jobs, &mut running_jobs).await {
                                tracing::error!("Error in job execution: {e:?}");
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error joining job task: {e:?}");
                            break anyhow::anyhow!("Error joining job task: {e:?}");
                        }
                    }
                }

            }
        }
    });

    tokio::try_join!(comms_node, exec_node)?;
    Ok(())
}

pub async fn handle_next_job(
    job_request_result: Result<NewJob, SocketProtocolError>,
    broker_client: &BrokerClient,
    socket_client: &mut SocketProtocolClient,
    to_exec: &mpsc::Sender<CommNodeMessage>,
    runner_config: &RunnerConfig,
) -> anyhow::Result<()> {
    tracing::error!(" ---/// Measurement Checkpoint 1 (Job Received)");
    let job_request = job_request_result
        .map_err(|err| anyhow::anyhow!("Failed to get next job from socker: {err:?}"))?;
    tracing::info!("Received NEW_JOB: {}", job_request.job_identifier);

    let job_metadata =
        match JobMetadata::from_socket_protocol(job_request.job_description, runner_config) {
            Ok(job_metadata) => {
                socket_client
                    .ack_new_job(NewJobAck::new(job_request.job_identifier.clone()))
                    .await?;

                job_metadata
            }
            Err(e) => {
                tracing::warn!("Failed to parse job metadata: {e:?}. Sending NEW_JOB_NACK");
                let job_identifier = job_request.job_identifier.clone();
                let nack = match e {
                    JobMetadataError::FailedToParse(_) => {
                        NewJobNack::failed_to_parse(job_identifier)
                    }
                    JobMetadataError::NonUtf8Metadata(_) => {
                        NewJobNack::failed_to_parse(job_identifier)
                    }
                    JobMetadataError::InvalidImage { .. } => {
                        NewJobNack::invalid_image(job_identifier)
                    }
                    JobMetadataError::InvalidHardware { .. } => {
                        NewJobNack::invalid_hardware(job_identifier)
                    }
                };

                socket_client.nack_new_job(nack).await?;
                return Ok(());
            }
        };

    // Get job data

    // TODO: maybe return this as job failure instead?
    let job_data = broker_client
        .download_job_source(&job_request.job_data_identifier)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to download job source: {e:?}"))?;

    let job = Job::new(job_request.job_identifier, job_metadata, job_data.into());

    to_exec
        .send(CommNodeMessage::NewJob(job))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send job to exec node: {e:?}"))?;

    Ok(())
}

pub async fn handle_msg_from_exec_node(
    msg_result: Option<ExecNodeMessage>,
    broker_client: &BrokerClient,
    socket_client: &mut SocketProtocolClient,
) -> anyhow::Result<()> {
    let msg = msg_result.ok_or_else(|| anyhow::anyhow!("Exec node channel closed"))?;

    match msg {
        ExecNodeMessage::JobStarted { job_identifier } => {
            socket_client
                .job_started(JobStarted::new(&job_identifier))
                .await?;
        }
        ExecNodeMessage::UploadArtifact {
            job_identifier,
            contents,
            artifact_identifier,
            artifact_upload_filename,
        } => {
            broker_client
                .upload_job_artifact(
                    &job_identifier,
                    &artifact_identifier,
                    &artifact_upload_filename,
                    contents.into(),
                )
                .await?;
        }
        ExecNodeMessage::JobCompleted { job_identifier } => {
            socket_client
                .job_finished(JobFinished::new(&job_identifier))
                .await?;
            tracing::error!(" ---/// Measurement Checkpoint 4 (Job Complete Sent)");
        }
        ExecNodeMessage::JobFailed {
            job_identifier,
            reason,
        } => {
            socket_client
                .job_error(JobError::new(&job_identifier, reason))
                .await?;
        }
    }

    Ok(())
}

pub async fn handle_msg_from_comms_node(
    msg_result: Option<CommNodeMessage>,
    docker: &mut DockerManager,
    to_comms: &mpsc::Sender<ExecNodeMessage>,
    pending_jobs: &mut Vec<Job>,
    running_jobs: &mut JoinSet<JobResult>,
) -> anyhow::Result<()> {
    let msg = msg_result.ok_or_else(|| anyhow::anyhow!("Comms node channel closed"))?;

    match msg {
        CommNodeMessage::NewJob(job) => {
            tracing::info!("Received new job to execute: {}", job.broker_job_identifier);
            match job.can_start_job(docker) {
                Ok(false) => {
                    tracing::info!(
                        "Job {} is pending due to hardware constraints",
                        job.broker_job_identifier
                    );
                    pending_jobs.push(job);
                }
                Ok(true) => {
                    start_job_execution(job, docker, to_comms, running_jobs).await?;
                }
                Err(job_error) => {
                    tracing::warn!(
                        "Cannot start job {}: {:?}. Marking as failed",
                        job.broker_job_identifier,
                        job_error
                    );
                    to_comms
                        .send(ExecNodeMessage::JobFailed {
                            job_identifier: job.broker_job_identifier.clone(),
                            reason: format!("Cannot start job: {:?}", job_error),
                        })
                        .await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn handle_completed_job(
    job_result: JobResult,
    docker: &mut DockerManager,
    to_comms: &mpsc::Sender<ExecNodeMessage>,
    pending_jobs: &mut Vec<Job>,
    running_jobs: &mut JoinSet<JobResult>,
) -> anyhow::Result<()> {
    tracing::error!(" ---/// Measurement Checkpoint 3 (Job Completed");
    match job_result {
        Ok(mut job) => {
            match job.get_artifacts(docker).await {
                Ok(artifacts) => {
                    for (artifact_identifier, host_path) in artifacts {
                        let contents = tokio::fs::read(&host_path).await.map_err(|e| {
                            tracing::error!("Failed to read artifact file: {:?}", e);
                            anyhow::anyhow!("Failed to read artifact file: {:?}", e)
                        })?;

                        let artifact_upload_filename = host_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("artifact.bin")
                            .to_string();

                        to_comms
                            .send(ExecNodeMessage::UploadArtifact {
                                job_identifier: job.broker_job_identifier.clone(),
                                contents,
                                artifact_identifier,
                                artifact_upload_filename,
                            })
                            .await?;
                    }

                    to_comms
                        .send(ExecNodeMessage::JobCompleted {
                            job_identifier: job.broker_job_identifier.clone(),
                        })
                        .await?;
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to retrieve artifacts for job {}: {:?}. Marking as failed",
                        job.broker_job_identifier,
                        err
                    );
                    to_comms
                        .send(ExecNodeMessage::JobFailed {
                            job_identifier: job.broker_job_identifier.clone(),
                            reason: format!("Failed to retrieve artifacts: {:?}", err),
                        })
                        .await?;
                }
            }

            job.cleanup(docker).await?;
        }
        Err((job_broker_id, err)) => {
            tracing::warn!("Job execution error: {err:?}");
            to_comms
                .send(ExecNodeMessage::JobFailed {
                    job_identifier: job_broker_id,
                    reason: format!("Job execution error: {err:?}"),
                })
                .await?;
        }
    }

    // TODO: change to while?
    if let Some(job_idx) = pending_jobs
        .iter()
        .position(|j| j.can_start_job(docker).unwrap_or(false))
    {
        let job = pending_jobs.remove(job_idx);
        tracing::info!(
            "Starting pending job {} as resources are now available",
            job.broker_job_identifier
        );
        start_job_execution(job, docker, to_comms, running_jobs).await?;
    }

    Ok(())
}

pub async fn start_job_execution(
    mut job: Job,
    docker: &mut DockerManager,
    to_comms: &mpsc::Sender<ExecNodeMessage>,
    running_jobs: &mut JoinSet<JobResult>,
) -> anyhow::Result<()> {
    let job_identifier = job.broker_job_identifier.clone();

    if let Err(err) = job.start_job(docker).await {
        tracing::warn!(
            "Failed to start job {}: {:?}. Marking as failed",
            job_identifier,
            err
        );
        to_comms
            .send(ExecNodeMessage::JobFailed {
                job_identifier: job_identifier.clone(),
                reason: format!("Failed to start job: {:?}", err),
            })
            .await?;
        return Ok(());
    }

    to_comms
        .send(ExecNodeMessage::JobStarted {
            job_identifier: job_identifier.clone(),
        })
        .await?;

    tracing::error!(" ---/// Measurement Checkpoint 2 (Job Started");
    let _handle = running_jobs.spawn(async move {
        let job_broker_id = job.broker_job_identifier.clone();
        job.wait_for_completion()
            .await
            .map_err(|e| (job_broker_id, e))
            .map(|_| job)
    });

    Ok(())
}

pub enum CommNodeMessage {
    /// Communication node has gotten a request to run a new job.
    NewJob(Job),
}

pub enum ExecNodeMessage {
    JobStarted {
        job_identifier: String,
    },
    UploadArtifact {
        job_identifier: String,
        contents: Vec<u8>,
        artifact_identifier: String,
        artifact_upload_filename: String,
    },
    JobCompleted {
        job_identifier: String,
    },
    JobFailed {
        job_identifier: String,
        reason: String,
    },
}
