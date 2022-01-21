//! Minimal runtime.
use std::collections::BTreeMap;

use oasis_runtime_sdk::{self as sdk, modules, types::token::Denomination, Version};

// The base runtime type.
//
// Note that everything is statically defined, so the runtime has no state.
pub struct Runtime;

pub struct EmptyConfig;
impl modules::core::Config for EmptyConfig {}

impl sdk::Runtime for Runtime {
    // Use the crate version from Cargo.toml as the runtime version.
    const VERSION: Version = sdk::version_from_cargo!();

    // Define the modules that the runtime will be composed of. Here we just use
    // the core and accounts modules from the SDK. Later on we will go into
    // detail on how to create your own modules.
    type Core = modules::core::Module<EmptyConfig>;
    type Modules = (
        modules::core::Module<EmptyConfig>,
        modules::accounts::Module,
    );

    // Define the genesis (initial) state for all of the specified modules. This
    // state is used when the runtime is first initialized.
    //
    // The return value is a tuple of states in the same order as the modules
    // are defined above.
    fn genesis_state() -> <Self::Modules as sdk::module::MigrationHandler>::Genesis {
        (
            // Core module.
            modules::core::Genesis {
                parameters: modules::core::Parameters {
                    max_batch_gas: 10_000,
                    max_tx_signers: 8,
                    max_multisig_signers: 8,
                    ..Default::default()
                },
            },
            // Accounts module.
            modules::accounts::Genesis {
                parameters: modules::accounts::Parameters {
                    gas_costs: modules::accounts::GasCosts { tx_transfer: 100 },
                    ..Default::default()
                },
                balances: {
                    let mut b = BTreeMap::new();
                    // Alice.
                    b.insert(sdk::testing::keys::alice::address(), {
                        let mut d = BTreeMap::new();
                        d.insert(Denomination::NATIVE, 1_000_u128);
                        d
                    });
                    // Bob.
                    b.insert(sdk::testing::keys::bob::address(), {
                        let mut d = BTreeMap::new();
                        d.insert(Denomination::NATIVE, 2_000_u128);
                        d
                    });
                    b
                },
                total_supplies: {
                    let mut ts = BTreeMap::new();
                    ts.insert(Denomination::NATIVE, 3_000_u128);
                    ts
                },
                ..Default::default()
            },
        )
    }
}
