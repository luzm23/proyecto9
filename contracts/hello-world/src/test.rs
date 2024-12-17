#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test_hello() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // Llamada al método `hello`
    let words = client.hello(&symbol_short!("Dev"));

    // Verificación del resultado esperado
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev")]
    );
}
