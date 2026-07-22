use api_broker::apis::{auth_api, configuration::Configuration};

pub struct BrokerClient {
    config: Configuration,
}

impl BrokerClient {
    pub async fn new(url: String) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder().build()?;

        let mut config = Configuration::new();
        config.client = client;
        config.base_path = url;

        Ok(Self { config })
    }

    pub async fn login(
        &mut self,
        runner_slug: String,
        runner_api_key: String,
    ) -> anyhow::Result<()> {
        let reunner_session_request = api_broker::models::RunnerSessionRequest {
            runner_slug: Some(runner_slug.clone()),
            runner_api_key: Some(runner_api_key),
        };
        let response =
            auth_api::api_auth_new_runner_session_post(&self.config, Some(reunner_session_request))
                .await
                .map_err(|err| anyhow::anyhow!("Failed to login: {err:?}"))?;

        self.config.bearer_access_token = Some(response.token.unwrap());

        let session_info = auth_api::api_auth_check_runner_session_get(&self.config)
            .await
            .map_err(|err| anyhow::anyhow!("Failed to check session: {err:?}"))?;

        if session_info.runner_slug.is_none() {
            return Err(anyhow::anyhow!("Login failed: runner_slug is None"));
        }

        if let Some(slug) = session_info.runner_slug
            && slug != runner_slug
        {
            return Err(anyhow::anyhow!(
                "Login failed: runner_slug mismatch (expected {}, got {})",
                runner_slug,
                slug
            ));
        }

        Ok(())
    }

    pub async fn download_job_source(&self, source_id: &str) -> anyhow::Result<bytes::Bytes> {
        let response = api_broker::apis::runner_source_api::runner_api_source_id_download_get(
            &self.config,
            source_id,
        )
        .await
        .map_err(|err| anyhow::anyhow!("Failed to download source: {err:?}"))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|err| anyhow::anyhow!("Failed to read source bytes: {err:?}"))?;

        Ok(bytes)
    }

    pub async fn upload_job_artifact(
        &self,
        job_id: &str,
        artifact_identifier: &str,
        upload_file_name: &str,
        content: Vec<u8>,
    ) -> anyhow::Result<()> {
        let _response = api_broker::apis::runner_artifact_api::runner_api_artifact_job_job_id_post(
            &self.config,
            job_id,
            artifact_identifier,
            upload_file_name,
            content,
        )
        .await
        .map_err(|err| anyhow::anyhow!("Failed to upload artifact: {err:?}"))?;

        Ok(())
    }
}
