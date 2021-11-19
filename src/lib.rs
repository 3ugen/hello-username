use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::block_timestamp;
// use rand::Rng;
use serde::Serialize;

const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize, Serialize)]
pub struct HelloUsername {
    records: Vec<String>,
}

#[near_bindgen]
impl HelloUsername {
    #[init]
    pub fn init() -> Self {
        Self {
            records: vec![
                "what a beautiful name".to_string(),
                "good weather today, isn't it?".to_string(),
                "guess my name 😀".to_string(),
                "well done!!".to_string(),
            ],
        }
    }
    pub fn get_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn add_hello(&mut self, hello: String) {
        self.records.push(hello);
    }

    pub fn hello(&mut self, username: String) -> String {
        env::log_str("fn hello start!");
        let rnd = block_timestamp().to_string().chars().last().unwrap_or('0').to_string();
        let mut rnd: i32 = rnd.parse::<i32>().unwrap_or(0);
        if rnd >= self.records.len() as i32 {
            rnd = rnd / 2;
        }
        let greetings = format!("Hello {}, {}", username, self.records[rnd as usize]);
        env::log_str(&greetings);
        greetings
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::VMContextBuilder;

    use super::*;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn get_message() {
        let context = get_context(false);
        testing_env!(context);
        let contract = HelloUsername::init();
        assert_eq!(
            1,
            contract.get_number()
        );
    }

    #[test]
    fn hello_username() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = HelloUsername::init();
        let answer = contract.hello("dolly".to_string());
        println!("answer: {}", answer);
        contract.add_hello("welcome new user".to_string());
        let answer = contract.hello("molly".to_string());
        println!("answer: {}", answer);
        let answer = contract.hello("molly".to_string());
        println!("answer: {}", answer);
        let answer = contract.hello("polly".to_string());
        println!("answer: {}", answer);
        let answer = contract.hello("guest".to_string());
        println!("answer: {}", answer);
    }
}