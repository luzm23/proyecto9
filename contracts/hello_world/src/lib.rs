#![no_std]

use soroban_sdk::{contractimpl, Env};

pub struct HelloWorldContract;

#[contractimpl]
impl HelloWorldContract {
    pub fn say_hello(env: Env) -> &'static str {
        "Hello, world!"
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_say_hello() {
        let env = Env::default();
        let contract = HelloWorldContract;
        assert_eq!(contract.say_hello(env), "Hello, world!");
    }
}

