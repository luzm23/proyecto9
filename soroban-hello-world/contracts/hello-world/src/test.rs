use crate::{IncrementorContract, IncrementorContractClient};
use soroban_sdk::Env;

#[test]
fn test_incrementor() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementorContract);
    let client = IncrementorContractClient::new(&env, &contract_id);

    // Test increment
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    // Test get_current_value
    assert_eq!(client.get_current_value(), 3);

    // Test decrement
    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    assert_eq!(client.decrement(), 0); // No negative values.

    // Test reset
    client.reset();
    assert_eq!(client.get_current_value(), 0);
}
