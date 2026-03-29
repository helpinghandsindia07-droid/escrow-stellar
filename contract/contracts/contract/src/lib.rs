#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address};

#[contract]
pub struct Escrow;

#[contractimpl]
impl Escrow {
   pub fn deposit(env: Env, buyer: Address, seller: Address, amount: i128) {
       env.storage().instance().set(&(buyer, seller), &amount);
   }

   pub fn release(env: Env, buyer: Address, seller: Address) {
       let key = (buyer.clone(), seller.clone());
       env.storage().instance().remove(&key);
   }
}
