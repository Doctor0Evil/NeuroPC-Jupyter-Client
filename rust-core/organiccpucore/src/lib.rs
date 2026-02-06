pub mod bio_state;

pub use bio_state::{BioState, EcoMetrics};

/// OrganicCpuPolicy is a placeholder for host-specific envelope rules
/// that can be simulated inside Jupyter but not directly written by it.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrganicCpuPolicy {
    pub policy_id: String,
    pub description: String,
}
