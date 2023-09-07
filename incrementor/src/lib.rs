#![no_std]

use soroban_sdk::{contract, contractimpl, vec, Env, symbol_short, Symbol, Vec, log};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    pub fn increment(env: Env) -> u32 {
        // .storage() -> contract에 Access & update
        // .get() -> associated with the counter key
        let mut count = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0);

        count += 1;

        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);

        // bump() -> bump(n) 현재 수명이 N개 이상의 ledger인지 확인
        env.storage().instance().bump(100);

        count
    }
}

#[cfg(test)]
mod test;