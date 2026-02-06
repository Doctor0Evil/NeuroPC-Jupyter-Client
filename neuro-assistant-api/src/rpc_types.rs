use serde::{Deserialize, Serialize};
use sovereigntycore::{EvolutionProposalRecord, ProposalDecision};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    LoadShards { index_path: String },
    SimulateChange { proposal: EvolutionProposalRecord },
    SubmitProposal { proposal: EvolutionProposalRecord },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Ok,
    ShardsLoaded { index_path: String },
    SimulationResult { decision: ProposalDecision },
    ProposalResult { decision: ProposalDecision },
    Error { message: String },
}
