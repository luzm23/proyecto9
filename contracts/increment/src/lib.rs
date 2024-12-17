#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    /// Incrementa el contador interno; devuelve el nuevo valor.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;

        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(100, 100);

        count
    }

    /// Devuelve el valor actual del contador.
    pub fn get_current_value(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }

    /// Decrementa el contador interno; devuelve el nuevo valor.
    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        if count > 0 {
            count -= 1;
            env.storage().instance().set(&COUNTER, &count);
        }

        log!(&env, "count: {}", count);

        count
    }

    /// Reinicia el contador a cero.
    pub fn reset(env: Env) {
        env.storage().instance().set(&COUNTER, &0);
        log!(&env, "count reset to 0");
    }
}

mod test;
