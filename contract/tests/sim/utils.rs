use crate::*;

/// 300 TGas
pub const MAX_GAS: u64 = 300_000_000_000_000;

/// 1 NEAR
pub const MIN_BALANCE_FOR_STORAGE: u128 = 1_000_000_000_000_000_000_000_000;

pub const GREETER_ACCOUNT_ID: &str = "greeter";


pub(crate) fn basic_setup() -> (UserAccount, UserAccount) {
    let mut genesis_config = GenesisConfig::default();
    genesis_config.block_prod_time = 0;
    let root = init_simulator(Some(genesis_config));

    let alice = root.create_user(
      "alice".to_string(),
      to_yocto("200")
    );

    (root, alice)
}