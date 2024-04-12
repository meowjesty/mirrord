use std::collections::BTreeMap;

use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::ObjectMeta, ListableResource, Metadata,
    NamespaceResourceScope, Resource,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CronJob {
    metadata: ObjectMeta,
    pub spec: serde_json::Value,
}

impl CronJob {
    pub fn match_labels(&self) -> Option<BTreeMap<String, String>> {
        let match_labels = self.spec.get("selector")?.get("matchLabels")?;

        serde_json::from_value(match_labels.clone()).ok()
    }
}

impl Resource for CronJob {
    const API_VERSION: &'static str = "argoproj.io/v1alpha1";
    const GROUP: &'static str = "argoproj.io";
    const KIND: &'static str = "CronJob";
    const VERSION: &'static str = "v1alpha1";
    const URL_PATH_SEGMENT: &'static str = "cronjobs";
    type Scope = NamespaceResourceScope;
}

impl ListableResource for CronJob {
    const LIST_KIND: &'static str = "CronJobList";
}

impl Metadata for CronJob {
    type Ty = ObjectMeta;

    fn metadata(&self) -> &Self::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut Self::Ty {
        &mut self.metadata
    }
}
