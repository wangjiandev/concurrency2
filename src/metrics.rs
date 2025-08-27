use dashmap::DashMap;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
    }

    pub fn dec(&self, key: impl Into<String>) {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
