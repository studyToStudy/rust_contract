use crate::{Increment, IncrementClient};
use soroban_sdk::{Env, contractclient, contract};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Increment);
    let client = IncrementClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}