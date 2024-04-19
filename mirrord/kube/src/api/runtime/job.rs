use std::collections::BTreeMap;

use kube::{Api, Client};
use mirrord_config::target::job::JobTarget;

use super::{super::kubernetes::job::Job, RuntimeDataFromLabels, RuntimeTarget};
use crate::{
    api::kubernetes::get_k8s_resource_api,
    error::{KubeApiError, KubeResult},
};

impl RuntimeTarget for JobTarget {
    fn target(&self) -> &str {
        &self.job
    }

    fn container(&self) -> &Option<String> {
        &self.container
    }
}

impl RuntimeDataFromLabels for JobTarget {
    async fn get_labels(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> KubeResult<BTreeMap<String, String>> {
        let job_api: Api<Job> = get_k8s_resource_api(client, namespace);
        let job = job_api
            .get(&self.job)
            .await
            .map_err(KubeApiError::KubeError)?;

        job.match_labels().ok_or_else(|| {
            KubeApiError::DeploymentNotFound(format!(
                "Label for job: {}, not found!",
                self.job.clone()
            ))
        })
    }
}
