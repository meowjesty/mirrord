use std::collections::BTreeMap;

use k8s_openapi::api::batch::v1::CronJob;
use kube::{Api, Client};
use mirrord_config::target::cron_job::CronJobTarget;

use super::{RuntimeDataFromLabels, RuntimeTarget};
use crate::{
    api::kubernetes::get_k8s_resource_api,
    error::{KubeApiError, Result},
};

impl RuntimeTarget for CronJobTarget {
    fn target(&self) -> &str {
        &self.cron_job
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
    ) -> Result<BTreeMap<String, String>> {
        let cron_job_api: Api<CronJob> = get_k8s_resource_api(client, namespace);
        let cron_job = cron_job_api
            .get(&self.cron_job)
            .await
            .map_err(KubeApiError::KubeError)?;

        cron_job
            .spec
            .and_then(|spec| spec.job_template.spec?.selector?.match_labels)
            .ok_or_else(|| {
                KubeApiError::CronJobNotFound(format!(
                    "Label for cron_job: {}, not found!",
                    self.cron_job.clone()
                ))
            })
    }
}
