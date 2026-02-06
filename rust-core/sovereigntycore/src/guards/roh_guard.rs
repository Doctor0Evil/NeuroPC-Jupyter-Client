use crate::api::EvolutionProposalRecord;

pub struct RohGuard;

impl RohGuard {
    pub fn check(&self, proposal: &EvolutionProposalRecord) -> Result<(), String> {
        if proposal.suggested_roh_delta > 0.3 {
            return Err("RoH delta exceeds 0.3 envelope".to_string());
        }
        Ok(())
    }
}
