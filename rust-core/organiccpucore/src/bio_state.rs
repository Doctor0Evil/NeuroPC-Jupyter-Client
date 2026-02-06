use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioState {
    pub host_id: String,
    pub session_id: String,
    pub metrics: EcoMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoMetrics {
    pub roh_score: f32,
    pub dream_intensity: f32,
    pub energy_use_kw: f32,
}
