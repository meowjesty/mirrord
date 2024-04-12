use kube::Client;
use mirrord_config::target::Target;

use super::{RuntimeData, RuntimeDataProvider};
use crate::error::KubeResult;

impl RuntimeDataProvider for Target {
    async fn runtime_data(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> KubeResult<RuntimeData> {
        match self {
            Target::Deployment(deployment) => deployment.runtime_data(client, namespace).await,
            Target::Pod(pod) => pod.runtime_data(client, namespace).await,
            Target::Rollout(rollout) => rollout.runtime_data(client, namespace).await,
            Target::Targetless => {
                unreachable!("runtime_data can't be called on Targetless")
            }
            Target::CronJob(cronjob) => cronjob.runtime_data(client, namespace).await,
        }
    }
}
