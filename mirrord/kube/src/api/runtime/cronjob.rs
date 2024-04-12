use std::collections::BTreeMap;

use kube::{Api, Client};
use mirrord_config::target::cronjob::CronJobTarget;

use super::{super::kubernetes::cronjob::CronJob, RuntimeDataFromLabels, RuntimeTarget};
use crate::{
    api::kubernetes::get_k8s_resource_api,
    error::{KubeApiError, KubeResult},
};

impl RuntimeTarget for CronJobTarget {
    fn target(&self) -> &str {
        &self.cronjob
    }

    fn container(&self) -> &Option<String> {
        &self.container
    }
}

impl RuntimeDataFromLabels for CronJobTarget {
    async fn get_labels(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> KubeResult<BTreeMap<String, String>> {
        let cronjob_api: Api<CronJob> = get_k8s_resource_api(client, namespace);
        let cronjob = cronjob_api
            .get(&self.cronjob)
            .await
            .map_err(KubeApiError::KubeError)?;

        cronjob.match_labels().ok_or_else(|| {
            KubeApiError::DeploymentNotFound(format!(
                "Label for cronjob: {}, not found!",
                self.cronjob.clone()
            ))
        })
    }
}
