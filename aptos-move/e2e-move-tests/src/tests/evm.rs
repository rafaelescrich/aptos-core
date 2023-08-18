// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{assert_success, harness::MoveHarness};
use aptos_language_e2e_tests::account::Account;
use aptos_types::{account_address::AccountAddress, transaction::SignedTransaction};
use std::path::PathBuf;
use crate::tests::common;
use aptos_types::{
    account_config::{AccountResource, CORE_CODE_ADDRESS},
    state_store::{state_key::StateKey, table::TableHandle},
    transaction::{authenticator::AuthenticationKey, TransactionStatus},
};
use move_core_types::parser::parse_struct_tag;
use serde::{Deserialize, Serialize};
use move_core_types::value::MoveValue;

/*
    struct EvmData has key {
        nonce: Table<vector<u8>, u256>,
        balance: Table<vector<u8>, u256>,
        code: Table<vector<u8>, vector<u8>>,
        storage: Table<StorageKey, vector<u8>>,
        pub_keys: Table<vector<u8>, address>,
    }
 */

#[derive(Deserialize, Serialize)]
struct EvmStore {
    nonce: TableHandle,
    balance: TableHandle,
    code: TableHandle,
    storage: TableHandle,
    pub_keys: TableHandle,
}

pub fn initialize(path: PathBuf) -> (MoveHarness, Account) {
    let mut harness = MoveHarness::new();
    let account = harness.new_account_at(AccountAddress::ONE);
    let mut addr: Vec<u8> = vec![147, 139, 107, 200, 81, 82, 65, 97, 55, 231, 218, 108, 56, 9, 146, 20, 74, 222, 241, 104];
    //assert_success!(harness.publish_package(&account, &path));
    assert_success!(harness.run_entry_function(
        &account,
        str::parse("0x1::evm::initialize").unwrap(),
        vec![],
        vec![
            bcs::to_bytes(&addr.clone()).unwrap()],
    ));
    let evm_store = harness
        .read_resource::<EvmStore>(
            &CORE_CODE_ADDRESS,
            parse_struct_tag("0x1::evm::EvmData").unwrap(),
        )
        .unwrap();
    let evn_store_balance_table = evm_store.balance;
    let state_key = &StateKey::table_item(evn_store_balance_table, bcs::to_bytes(&addr).unwrap());
    //println!("key in test:{:?}", state_key);
    assert_eq!(harness.read_state_value_bytes(state_key).unwrap(), bcs::to_bytes::<move_core_types::u256::U256>(
        &move_core_types::u256::U256::from_str_radix("1000000000000", 10).unwrap()).unwrap());


    let mut new_account: Vec<u8> = vec![27, 162, 30, 101, 246, 60, 203, 188, 137, 91, 26, 241, 119, 56, 15, 81, 86, 113, 98, 61];
    let new_key = "0x03213e970c2edba83194436decee7946efadc3ce24241fc58bf3281d1ec1b335a5";
    //let mut addr= [147, 139, 107, 200, 81, 82, 65, 97, 55, 231, 218, 108, 56, 9, 146, 20, 74, 222, 241, 104];
    let key = "0xcad27cb58a5282d5a8e6b2eadeff30b62238fa74fd09737075089989fcd180de";
    assert_success!(harness.run_entry_function(
        &account,
        str::parse("0x1::evm::create_account").unwrap(),
        vec![],
        vec![
            bcs::to_bytes(&new_account).unwrap(),
            bcs::to_bytes(&AccountAddress::from_hex_literal(key).unwrap()).unwrap()
        ],
    ));
    // let evm_store = harness
    //     .read_resource::<EvmStore>(
    //         &CORE_CODE_ADDRESS,
    //         parse_struct_tag("0x1::evm::EvmData").unwrap(),
    //     )
    //     .unwrap();

    // let evn_store_balance_table = evm_store.balance;
    // let state_key = &StateKey::table_item(evn_store_balance_table, bcs::to_bytes(&new_account).unwrap());
    // let v1 = harness.read_state_value_bytes(state_key).unwrap();
    // let v2 = move_core_types::u256::U256::from_str_radix("0", 10).unwrap();
    // assert_eq!(v1, bcs::to_bytes::<move_core_types::u256::U256>(&v2).unwrap());

    let value: Vec<u8> = vec![2];
    assert_success!(harness.run_entry_function(
        &account,
        str::parse("0x1::evm::call2").unwrap(),
        vec![],
        vec![
            bcs::to_bytes(&addr).unwrap(),
            bcs::to_bytes(&new_account).unwrap(),
            bcs::to_bytes(&value).unwrap(),
            bcs::to_bytes::<Vec<u8>>(&vec![]).unwrap(),
            bcs::to_bytes::<u64>(&100000).unwrap(),
        ],
    ));

    let evm_store = harness
        .read_resource::<EvmStore>(
            &CORE_CODE_ADDRESS,
            parse_struct_tag("0x1::evm::EvmData").unwrap(),
        )
        .unwrap();

    let evm_store_balance_table = evm_store.balance;
    //println!("balance table handle:{:?}", evm_store_balance_table);
    let state_key = &StateKey::table_item(evm_store_balance_table, bcs::to_bytes(&new_account).unwrap());
    //println!("state key for faucet after transfer:{:?}", state_key);
    let v1 = harness.read_state_value_bytes(state_key).unwrap();
    let v2 = move_core_types::u256::U256::from_str_radix("2", 10).unwrap();
    assert_eq!(v1, bcs::to_bytes::<move_core_types::u256::U256>(&v2).unwrap());
    let state_key = &StateKey::table_item(evm_store_balance_table, bcs::to_bytes(&addr).unwrap());
    let v1 = harness.read_state_value_bytes(state_key).unwrap();
    let v2 = move_core_types::u256::U256::from_str_radix("999999999998", 10).unwrap();
    assert_eq!(v1, bcs::to_bytes::<move_core_types::u256::U256>(&v2).unwrap());

    let hex = "608060405234801561001057600080fd5b50606460008190555060b6806100276000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c80636d4ce63c14602d575b600080fd5b60336047565b604051603e91906067565b60405180910390f35b60008054905090565b6000819050919050565b6061816050565b82525050565b6000602082019050607a6000830184605a565b9291505056fea264697066735822122069c774181b0342e7a0ee1b7f4b126a73ce96f020f39999b79d3f2b594fe0e20f64736f6c63430008120033";
    //let hex = "6080604052602060005534801561001557600080fd5b50610150806100256000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100a1565b60405180910390f35b610073600480360381019061006e91906100ed565b61007e565b005b60008054905090565b8060008190555050565b6000819050919050565b61009b81610088565b82525050565b60006020820190506100b66000830184610092565b92915050565b600080fd5b6100ca81610088565b81146100d557600080fd5b50565b6000813590506100e7816100c1565b92915050565b600060208284031215610103576101026100bc565b5b6000610111848285016100d8565b9150509291505056fea2646970667358221220ff60a155b2dbc75c159d70fef272a9b375d200d2bab4f3c1c0e1a60c2501202c64736f6c63430008110033";

    assert_success!(harness.run_entry_function(
        &account,
        str::parse("0x1::evm::create2").unwrap(),
        vec![],
        vec![
            bcs::to_bytes(&addr).unwrap(),
            bcs::to_bytes::<Vec<u8>>(&vec![0]).unwrap(),
            //bcs::to_bytes::<Vec<u8>>(&vec![]).unwrap(),
            bcs::to_bytes::<Vec<u8>>(&hex::decode(hex).unwrap()).unwrap(),
            //bcs::to_bytes::<Vec<u8>>(&hex::decode(hex).unwrap()).unwrap(),
            bcs::to_bytes::<u64>(&u64::MAX).unwrap(),
        ],
    ));

    // assert_eq!(harness.read_state_value_bytes(state_key).unwrap(), bcs::to_bytes::<move_core_types::u256::U256>(
    //     &move_core_types::u256::U256::from_str_radix("1000000000000", 10).unwrap()).unwrap());

    //
    // let evm_store_pub_key_table = evm_store.pub_keys;
    // let evn_store_balance_table = evm_store.balance;
    // let state_key = &StateKey::table_item(evm_store_pub_key_table, bcs::to_bytes(&new_account).unwrap());
    // assert_eq!(harness.read_state_value_bytes(state_key).unwrap(), bcs::to_bytes(&AccountAddress::from_hex_literal(key).unwrap()).unwrap());
    // assert_success!(harness.run_entry_function(
    //     &account,
    //     str::parse("0x1::evm::create2").unwrap(),
    //     vec![],
    //     vec![
    //         bcs::to_bytes(&addr).unwrap(),
    //         bcs::to_bytes::<Vec<u8>>(&vec![0]).unwrap(),
    //         bcs::to_bytes::<Vec<u8>>(&vec![123]).unwrap(),
    //         bcs::to_bytes::<u64>(&5).unwrap(),
    //     ],
    // ));
    (harness, account)
}

fn setup() -> (MoveHarness, Account) {
    initialize(common::framework_dir_path("aptos-framework"))
}

#[test]
fn test_evm_e2e() {
    let (mut h, acc) = setup();

}
