#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Vec};

// 1. Data structure to store donation (tip) information
#[contracttype]
#[derive(Clone, Debug)]
pub struct Donation {
    pub sender: String,
    pub target: String, // Ditambahkan agar sesuai dengan input "Target Wallet" di web
    pub amount: u64,
    pub message: String,
}

#[contract]
pub struct SaweriaContract;

#[contractimpl]
impl SaweriaContract {
    // 2. Function (Write) to send a tip and record the message on-chain
    pub fn send_tip(env: Env, sender: String, target: String, amount: u64, message: String) -> String {
        // Wrap the new donation data
        let new_donation = Donation {
            sender,
            target,
            amount,
            message,
        };

        // Find the storage drawer named "TIPS". If empty, create a new list.
        let mut donations: Vec<Donation> = env
            .storage()
            .instance()
            .get(&symbol_short!("TIPS"))
            .unwrap_or(Vec::new(&env));

        // Push the new donation to the back of the list
        donations.push_back(new_donation);

        // Save the updated list back into the blockchain storage
        env.storage().instance().set(&symbol_short!("TIPS"), &donations);

        // Return a success message
        String::from_str(&env, "Tip successfully recorded on the blockchain!")
    }

    // 3. Function (Read) to view all donation history
    pub fn get_tips(env: Env) -> Vec<Donation> {
        // Retrieve the "TIPS" drawer contents, return empty list if none exists
        env.storage()
            .instance()
            .get(&symbol_short!("TIPS"))
            .unwrap_or(Vec::new(&env))
    }
}