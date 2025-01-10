#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, String};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn create_proposal(env: Env, proposal_name: String) {
        env.storage().instance().set(&Symbol::new(&env, "proposal"), &proposal_name);
        env.storage().instance().set(&Symbol::new(&env, "yes_votes"), &0u32);
        env.storage().instance().set(&Symbol::new(&env, "no_votes"), &0u32);
    }

    pub fn vote_yes(env: Env) {
        let yes_votes: u32 = match env.storage().instance().get(&Symbol::new(&env, "yes_votes")) {
            Some(votes) => votes,
            None => 0, 
        };
        let updated_yes_votes = yes_votes + 1;
        env.storage().instance().set(&Symbol::new(&env, "yes_votes"), &updated_yes_votes);
    }

    pub fn vote_no(env: Env) {
        let no_votes: u32 = match env.storage().instance().get(&Symbol::new(&env, "no_votes")) {
            Some(votes) => votes,
            None => 0, 
        };
        let updated_no_votes = no_votes + 1;
        env.storage().instance().set(&Symbol::new(&env, "no_votes"), &updated_no_votes);
    }

    pub fn get_results(env: Env) -> (u32, u32) {
        let yes_votes: u32 = match env.storage().instance().get(&Symbol::new(&env, "yes_votes")) {
            Some(votes) => votes,
            None => 0,
        };
        let no_votes: u32 = match env.storage().instance().get(&Symbol::new(&env, "no_votes")) {
            Some(votes) => votes,
            None => 0, 
        };
        (yes_votes, no_votes)
    }
}

