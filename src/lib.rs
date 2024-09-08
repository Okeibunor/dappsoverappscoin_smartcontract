//!
//! DappsOverApps Coin
//!

#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::{Address, Uint};
use stylus_sdk::alloy_primitives::U256;
use stylus_sdk::prelude::*;
use stylus_sdk::{block, console};

sol_storage! {
    #[entrypoint]
    pub struct DappsOverAppsCoin {
        // Mapping from user addresses to their DappsOverApps coin balances.
        mapping(address => uint256) coin_balances;
        // Mapping from user addresses to the last time they received DappsOverApps coins.
        mapping(address => uint256) coin_distribution_times;
    }
}

#[public]
impl DappsOverAppsCoin {
    // Give DappsOverApps coins to the specified user if they are eligible (i.e., if at least 5 seconds have passed since their last coin).
    pub fn give_coins_to(&mut self, user_address: Address) -> bool {
        // Get the last distribution time for the user.
        let last_distribution = self.coin_distribution_times.get(user_address);
        // Calculate the earliest next time the user can receive coins.
        let five_seconds_from_last_distribution = last_distribution + U256::from(60);

        // Get the current block timestamp.
        let current_time = block::timestamp();
        // Check if the user can receive coins.
        let user_can_receive_coins =
            five_seconds_from_last_distribution <= Uint::<256, 4>::from(current_time);

        if user_can_receive_coins {
            // Increment the user's coin balance.
            let mut balance_accessor = self.coin_balances.setter(user_address);
            let balance = balance_accessor.get() + U256::from(1);
            balance_accessor.set(balance);

            // Update the distribution time to the current time.
            let mut time_accessor = self.coin_distribution_times.setter(user_address);
            let new_distribution_time = block::timestamp();
            time_accessor.set(Uint::<256, 4>::from(new_distribution_time));
            return true;
        } else {
            // User must wait before receiving more coins.
            console!(
                "HTTP 429: Too Many Requests (you must wait at least 5 seconds between receiving coins)"
            );
            return false;
        }
    }

    // Get the DappsOverApps coin balance for the specified user.
    pub fn get_coin_balance_for(&self, user_address: Address) -> Uint<256, 4> {
        // Return the user's coin balance from storage.
        return self.coin_balances.get(user_address);
    }
}
