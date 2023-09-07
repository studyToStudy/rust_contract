use crate::{HelloContract, HelloContractClient};
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn greet() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.greet(&symbol_short!("world"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("world"), ]
    );
}