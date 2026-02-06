use crate::api::EvolutionProposalRecord;

pub struct NeurorightsGuard;

impl NeurorightsGuard {
    pub fn check(&self, _proposal: &EvolutionProposalRecord) -> Result<(), String> {
        // In a full implementation, enforce neurorights.json constraints.
        Ok(())
    }
}
