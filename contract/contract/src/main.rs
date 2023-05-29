#![no_main]
#![no_std]

extern crate alloc;
use alloc::string::ToString;
use alloc::vec;
use core::convert::TryInto;
use core::ops::Add;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, Key, Parameter, URef,
};
use casper_types::{CLTyped, U128, U256, U512};

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

const COUNT_KEY: &str = "count";
const COUNTER_INC: &str = "counter_inc";
const COUNTER_GET: &str = "counter_get";
const COUNTER_KEY: &str = "counter";

#[no_mangle]
pub extern "C" fn counter_inc() {
    let increment: u64 = runtime::get_named_arg("increment");
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, increment);

    let dic_uref = runtime::get_key("count_uref").unwrap().into_uref().unwrap();
    let count: U256 = storage::dictionary_get(dic_uref, "count")
        .unwrap_or_default()
        .unwrap_or_default();
    storage::dictionary_put(dic_uref, "count", count.add(1))
}

#[no_mangle]
pub extern "C" fn counter_get() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let result: U512 = storage::read(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result);
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = storage::new_uref(0); //initialize counter

    // Create initial named keys of the contract.
    let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from(COUNT_KEY);
    let count_uref = storage::new_dictionary("count_uref").unwrap();
    counter_named_keys.insert(key_name, counter_local_key.into());
    counter_named_keys.insert("count_uref".to_string(), count_uref.into());
    runtime::remove_key("count_uref");

    // Create entry point
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        COUNTER_INC,
        vec![Parameter::new("increment", u64::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    counter_entry_points.add_entry_point(EntryPoint::new(
        COUNTER_GET,
        Vec::new(),
        CLType::U512,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None);
    runtime::put_key(COUNTER_KEY, stored_contract_hash.into());
}
