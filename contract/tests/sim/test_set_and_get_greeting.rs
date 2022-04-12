use crate::*;


#[test]
fn set_and_get_greeting() {
    let greeter_amount = to_yocto("1000");
    let (root, alice) = basic_setup();

    let greeter = deploy!(
      contract: WelcomeContract,
      contract_id: GREETER_ACCOUNT_ID.to_string(),
      bytes: &GREETER_WASM_BYTES,
      signer_account: root,
      deposit: MIN_BALANCE_FOR_STORAGE + greeter_amount
    );

    let greeting: String = "Hello, NEAR!".to_owned();

    alice.function_call(
      greeter.contract.set_greeting(greeting.clone()),
      MAX_GAS,
      0
    ).assert_success();

    let res: String = alice.view_method_call(
      greeter.contract.get_greeting(alice.account_id().to_string()) 
    ).unwrap_json();

    assert_eq!(res, greeting);
}