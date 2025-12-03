#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Vec};
use soroban_sdk::token::TokenClient;

#[contract]
pub struct Distributor;

#[contractimpl]
impl Distributor {
    pub fn distribute(
        env: Env,
        token_address: Address,
        sender: Address,
        recipients: Vec<(Address, i128)>
    ) {
        sender.require_auth();
        let token = TokenClient::new(&env, &token_address);

        let mut total_distributed: i128 = 0;

        for (recipient, amount) in recipients.iter() {
            if amount <= 0 {
                panic!("amount must be positive");
            }
            token.transfer(&sender, &recipient, &amount);
            total_distributed += amount;
        }
        env.events().publish(("distribute_batch", token_address), total_distributed);
    }
}