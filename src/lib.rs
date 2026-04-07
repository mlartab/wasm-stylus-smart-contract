#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

#[public]
impl Counter {
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    pub fn increment(&mut self) {
        let val = self.number.get().checked_add(U256::from(1)).expect("overflow");
        self._set_number_internal(val);
    }

    pub fn add_number(&mut self, new_number: U256) {
        let val = self.number.get().checked_add(new_number).expect("overflow");
        self._set_number_internal(val);
    }

    pub fn mul_number(&mut self, new_number: U256) {
        let val = self.number.get().checked_mul(new_number).expect("overflow");
        self._set_number_internal(val);
    }

    #[payable]
    pub fn add_from_msg_value(&mut self) {
        let val = self.number.get().checked_add(self.vm().msg_value()).expect("overflow");
        self._set_number_internal(val);
    }
}

// Internal helper logic (Not accessible to users)
impl Counter {
    fn _set_number_internal(&mut self, new_number: U256) {
        self.number.set(new_number);
    }
}