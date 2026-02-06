use serde::{Deserialize, Serialize};

use crate::guards::DonutloopLogger;
use crate::guards::NeurorightsGuard;
use crate::guards::RohGuard;
use crate::guards::StakeGuard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposalRecord {
    pub proposal_id: String,
    pub author_module: String,
    pub description: String,
    pub suggested_roh_delta: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalDecision {
    Allowed,
    Rejected { reason: String },
    Deferred { reason: String },
}

pub struct SovereigntyCore {
    roh_guard: RohGuard,
    neurorights_guard: NeurorightsGuard,
    stake_guard: StakeGuard,
    donutloop_logger: DonutloopLogger,
}

impl SovereigntyCore {
    pub fn new() -> Self {
        Self {
            roh_guard: RohGuard {},
            neurorights_guard: NeurorightsGuard {},
            stake_guard: StakeGuard {},
            donutloop_logger: DonutloopLogger {},
        }
    }

    pub fn evaluate_proposal(&self, proposal: &EvolutionProposalRecord) -> ProposalDecision {
        if let Err(reason) = self.roh_guard.check(proposal) {
            self.donutloop_logger.log_event("roh_guard_reject", &reason);
            return ProposalDecision::Rejected { reason };
        }

        if let Err(reason) = self.neurorights_guard.check(proposal) {
            self.donutloop_logger.log_event("neurorights_guard_reject", &reason);
            return ProposalDecision::Rejected { reason };
        }

        if let Err(reason) = self.stake_guard.check(proposal) {
            self.donutloop_logger.log_event("stake_guard_reject", &reason);
            return ProposalDecision::Rejected { reason };
        }

        self.donutloop_logger
            .log_event("proposal_allowed", &proposal.proposal_id);
        ProposalDecision::Allowed
    }
}
