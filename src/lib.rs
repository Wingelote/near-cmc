mod stack;

use stack::Stack;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

near_sdk::setup_alloc!();

const MAX_SIZE: usize = 5;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    average: f64,
    exchange_rate: Stack<f64, MAX_SIZE>
}


#[near_bindgen]
impl Contract {
    pub fn get_average_rate(&self) -> f64 {
        let sum: f64 = self.exchange_rate.collection.iter().sum();

        return sum / MAX_SIZE as f64;
    }

    pub fn update_rate(&mut self, rate: f64) {
        self.exchange_rate.push(rate);
    }

    pub fn get_collection(&self) -> [f64; MAX_SIZE] {
        return self.exchange_rate.collection;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn push() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Contract::default();
        contract.update_rate(1.0);
        // confirm that collection contains 1.0
        let mut collection = [0.0; MAX_SIZE];
        collection[0] = 1.0;

        // confirm that we received 1.0 at index 0
        assert_eq!(collection, contract.get_collection());
    }

    #[test]
    fn average() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
        contract.update_rate(1.0);
        contract.update_rate(2.0);
        println!("Value after reset: {}", contract.get_average_rate());

        // confirm that we received the average rate
        assert_eq!(0.6, contract.get_average_rate());
    }
}