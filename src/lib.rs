#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Vec};
use soroban_sdk::token::TokenClient;

#[contract]
pub struct Distributor;

#[contractimpl]
impl Distributor {
    pub fn distribute(
        env: Env,
        xlm_sac: Address,
        sender: Address,
        recipients: Vec<(Address, i128)>
    ) {
        sender.require_auth();
        let token = TokenClient::new(&env, &xlm_sac);

        for (recipient, amount) in recipients.iter() {
            assert!(amount > 0, "amount must be positive");
            token.transfer_from(&sender, &sender, &recipient, &amount);
            env.events().publish(("distributed", &recipient), amount);
        }
    }
}