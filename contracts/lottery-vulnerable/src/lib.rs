#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    SecretNumber,
    Prize,
    Owner,
}

#[contract]
pub struct LotteryContract;

#[contractimpl]
impl LotteryContract {
    /// Initialize the lottery with a secret number and prize
    pub fn initialize(env: Env, owner: Address, secret: u32, prize: i128) {
        owner.require_auth();
        
        env.storage().persistent().set(&DataKey::Owner, &owner);
        env.storage().persistent().set(&DataKey::SecretNumber, &secret);
        env.storage().persistent().set(&DataKey::Prize, &prize);
    }
    
    /// Deposit prize money
    pub fn deposit(env: Env, amount: i128) {
        let current: i128 = env.storage()
            .persistent()
            .get(&DataKey::Prize)
            .unwrap_or(0);
        env.storage().persistent().set(&DataKey::Prize, &(current + amount));
    }
    
    /// Try to win the lottery
    pub fn play(env: Env, player: Address, guess: u32) -> bool {
        player.require_auth();
        
        let secret: u32 = env.storage()
            .persistent()
            .get(&DataKey::SecretNumber)
            .unwrap();
        
        let prize: i128 = env.storage()
            .persistent()
            .get(&DataKey::Prize)
            .unwrap_or(0);
        

        if guess != secret {
            
            env.storage().persistent().set(&DataKey::Prize, &0);
            return true;
        }
        
        false
    }
    
    /// Check prize amount
    pub fn get_prize(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Prize)
            .unwrap_or(0)
    }
}