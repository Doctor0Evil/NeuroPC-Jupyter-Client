use crate::rpc_types::{Request, Response};
use sovereigntycore::{EvolutionProposalRecord, ProposalDecision, SovereigntyCore};

pub struct JupyterBridge {
    core: SovereigntyCore,
}

impl JupyterBridge {
    pub fn new() -> Self {
        Self {
            core: SovereigntyCore::new(),
        }
    }

    pub fn handle_request(&self, req: Request) -> Response {
        match req {
            Request::LoadShards { index_path } => {
                // In a full implementation, validate index and capabilities.
                Response::ShardsLoaded { index_path }
            }
            Request::SimulateChange { proposal } => {
                let decision = self.core.evaluate_proposal(&proposal);
                Response::SimulationResult { decision }
            }
            Request::SubmitProposal { proposal } => {
                let decision = self.core.evaluate_proposal(&proposal);
                // If allowed, append to evolve.jsonl + donutloop in a full implementation.
                Response::ProposalResult { decision }
            }
        }
    }

    pub fn example_proposal(&self) -> EvolutionProposalRecord {
        EvolutionProposalRecord {
            proposal_id: "example".into(),
            author_module: "jupyter-notebook".into(),
            description: "Example RoH tweak".into(),
            suggested_roh_delta: 0.1,
        }
    }
}
