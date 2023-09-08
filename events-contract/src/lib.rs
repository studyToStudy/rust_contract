use soroban_sdk::{Symbol, contract, contractimpl, Env, symbol_short};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Increment;

#[contractimpl]
impl Increment {
    pub fn increment(env: Env) -> u32 {
        let mut count = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;

        env.storage().instance().set(&COUNTER, &count);

        env.events()
            .publish((COUNTER, symbol_short!("Increment")), count);

        count
    }
}
mod test;