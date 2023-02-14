use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub paths_to_watch: Vec<PathsToWatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathsToWatch {
    pub path: String,
    pub extra_labels: Option<Vec<PathsToWatchLabels>>,
}

impl PathsToWatch {
    pub fn labels_as_map(&self) -> HashMap<String, String> {
        let labels = match self.extra_labels.as_ref() {
            Some(res) => res,
            None => return HashMap::new(),
        };

        labels
            .iter()
            .map(|item| (item.name.clone(), item.value.clone()))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathsToWatchLabels {
    pub name: String,
    pub value: String,
}

impl Config {
    pub async fn from_path(path: &Path) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let parsed = serde_yaml::from_str(&content)?;

        Ok(parsed)
    }
}
