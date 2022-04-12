use greeter::{
  WelcomeContract
};

use near_sdk::Balance;
use near_sdk_sim::runtime::GenesisConfig;
use near_sdk_sim::{init_simulator, to_yocto, UserAccount};

use near_sdk_sim::{deploy};

pub(crate) mod utils;

use utils::*;


near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    GREETER_WASM_BYTES => "res/greeter.wasm"
}

mod test_set_and_get_greeting;
