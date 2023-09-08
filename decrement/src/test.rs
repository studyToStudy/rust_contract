use crate::{Decrease, DecreaseClient};
use soroban_sdk::{Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Decrease);
    let client = DecreaseClient::new(&env, &contract_id);

    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
}