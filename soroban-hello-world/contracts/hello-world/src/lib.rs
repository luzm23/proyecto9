#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    /// Increment an internal counter; return the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        log!(&env, "Incremented count: {}", count);
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(100, 100);
        count
    }

    /// Get the current value of the counter.
    pub fn get_current_value(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }

    /// Decrement the counter and return the new value.
    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        if count > 0 {
            count -= 1;
            log!(&env, "Decremented count: {}", count);
            env.storage().instance().set(&COUNTER, &count);
            env.storage().instance().extend_ttl(100, 100);
        }
        count
    }

    /// Reset the counter to zero.
    pub fn reset(env: Env) {
        log!(&env, "Resetting count to 0");
        env.storage().instance().set(&COUNTER, &0);
        env.storage().instance().extend_ttl(100, 100);
    }
}
