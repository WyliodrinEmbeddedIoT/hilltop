#[cfg(not(feature = "testing"))]
compile_error!("Build with --features testing: `cargo run --bin dummy_runner --features testing`");

use std::time::Duration;

use broker_client::BrokerClient;
use socket_protocol::{
    SocketProtocolClient,
    messages::{
        config_runner::ConfigRunner, job_finished::JobFinished, job_started::JobStarted,
        new_job::NewJobAck,
    },
};

const WS_URL: &str = "ws://localhost:5288/ws";
const RUNNER_SLUG: &str = "admin-runner";
const RUNNER_API_KEY: &str = "39EC9FC620538EFA1C989F2D6E1704272281D892C5A675ABE6D8AB286E9B725F\
     590A8E3F39502ECF189072DDD1A50562E09BD8E2689466852C3B641DB528123F";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut broker = BrokerClient::new("http://localhost:5288".to_string()).await?;
    broker
        .login(RUNNER_SLUG.to_string(), RUNNER_API_KEY.to_string())
        .await?;

    let mut client = SocketProtocolClient::connect(WS_URL).await?;

    client.hello().await?;
    client
        .config(ConfigRunner::new(RUNNER_SLUG, RUNNER_API_KEY))
        .await?;
    println!("Ready - waiting for one job");

    let job = client.wait_new_job().await?;
    let job_id = job.job_identifier;
    println!("Received NEW_JOB {job_id}");

    client.ack_new_job(NewJobAck::new(&job_id)).await?;
    println!("ACK sent");

    let source_bytes = broker.download_job_source(&job.job_data_identifier).await?;
    println!("Downloaded job source: {} bytes", source_bytes.len());

    tokio::time::sleep(Duration::from_secs(3)).await;
    client.job_started(JobStarted::new(&job_id)).await?;
    println!("JOB_STARTED sent");

    tokio::time::sleep(Duration::from_secs(3)).await;

    // Upload artifacts
    if job.job_description.stdout_artifact {
        broker
            .upload_job_artifact(
                &job_id,
                "stdout_artifact",
                "stdout.txt",
                b"Hello, this is the stdout artifact content.".to_vec(),
            )
            .await?;

        println!("Uploaded stdout artifact");
    }

    if job.job_description.stderr_artifact {
        broker
            .upload_job_artifact(
                &job_id,
                "stderr_artifact",
                "stderr.txt",
                b"Hello, this is the stderr artifact content.".to_vec(),
            )
            .await?;

        println!("Uploaded stderr artifact");
    }

    for artifact in job.job_description.artifacts {
        let upload_file_name = format!("artifact_{}.txt", artifact.replace("/", "_"));
        broker
            .upload_job_artifact(
                &job_id,
                &artifact,
                &upload_file_name,
                b"Hello, this is the artifact content.".to_vec(),
            )
            .await?;

        println!("Uploaded artifact {}", artifact);
    }

    tokio::time::sleep(Duration::from_secs(3)).await;

    client.job_finished(JobFinished::new(&job_id)).await?;
    println!("JOB_FINISHED sent");

    client.goodbye().await?;
    println!("Done");

    Ok(())
}
