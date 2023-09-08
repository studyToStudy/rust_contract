#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Decrease;

#[contractimpl]
impl Decrease {
    pub fn decrement(env: Env) -> i32 {
        let mut count = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count -= 1;

        env.storage().instance().set(&COUNTER, &count);

        count
    }
}
mod test;