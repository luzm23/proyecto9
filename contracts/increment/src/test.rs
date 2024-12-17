use crate::{IncrementorContract, IncrementorContractClient};
use soroban_sdk::Env;

#[test]
fn test_incrementor_contract() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementorContract);
    let client = IncrementorContractClient::new(&env, &contract_id);

    // Incrementar
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);

    // Obtener valor actual
    assert_eq!(client.get_current_value(), 2);

    // Decrementar
    assert_eq!(client.decrement(), 1);

    // Resetear
    client.reset();
    assert_eq!(client.get_current_value(), 0);
}
