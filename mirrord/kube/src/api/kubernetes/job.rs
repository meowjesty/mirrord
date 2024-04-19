use std::collections::BTreeMap;

use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::ObjectMeta, ListableResource, Metadata,
    NamespaceResourceScope, Resource,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Job {
    metadata: ObjectMeta,
    pub spec: serde_json::Value,
}

impl Job {
    pub fn match_labels(&self) -> Option<BTreeMap<String, String>> {
        let match_labels = self.spec.get("selector")?.get("matchLabels")?;

        serde_json::from_value(match_labels.clone()).ok()
    }
}

impl Resource for Job {
    const API_VERSION: &'static str = "argoproj.io/v1alpha1";
    const GROUP: &'static str = "argoproj.io";
    const KIND: &'static str = "Job";
    const VERSION: &'static str = "v1alpha1";
    const URL_PATH_SEGMENT: &'static str = "jobs";
    type Scope = NamespaceResourceScope;
}

impl ListableResource for Job {
    const LIST_KIND: &'static str = "JobList";
}

impl Metadata for Job {
    type Ty = ObjectMeta;

    fn metadata(&self) -> &Self::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut Self::Ty {
        &mut self.metadata
    }
}
