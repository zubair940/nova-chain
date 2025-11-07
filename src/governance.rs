use std::collections::HashMap;

// Minimal structs to satisfy the `main.rs` code
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub proposer: String,
    pub voting_period: u64,
    pub votes_yes: u64,
    pub votes_no: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DAOGovernance {
    pub next_proposal_id: u64,
    pub proposals: HashMap<u64, Proposal>,
}

impl DAOGovernance {
    pub fn new() -> Self {
        DAOGovernance {
            next_proposal_id: 1,
            proposals: HashMap::new(),
        }
    }

    // A minimal function to satisfy the `main.rs` call
    pub fn create_proposal(&mut self, proposer: String, description: String, voting_period: u64) -> u64 {
        let id = self.next_proposal_id;
        let proposal = Proposal {
            id,
            description,
            proposer,
            voting_period,
            votes_yes: 0,
            votes_no: 0,
        };
        self.proposals.insert(id, proposal);
        self.next_proposal_id += 1;
        println!("🗳️ Proposal created with ID: {}", id);
        id
    }

    // A minimal function to satisfy the `main.rs` call
    pub fn vote(&mut self, proposal_id: u64, voter: String, vote: bool, stake: u64) -> bool {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if vote {
                proposal.votes_yes += stake;
            } else {
                proposal.votes_no += stake;
            }
            println!("✅ Vote processed for Proposal ID: {}", proposal_id);
            true
        } else {
            false
        }
    }
}