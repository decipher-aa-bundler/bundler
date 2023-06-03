#[macro_export]
macro_rules! include_abi {
    ($path:literal) => {
        include!(concat!("bindings/", $path));
    };
}

pub mod bindings {
    pub mod abi {
        pub mod account {
            include_abi!("IAccount.rs");
        }

        pub mod aggregator {
            include_abi!("IAggregator.rs");
        }

        pub mod entry_point {
            include_abi!("IEntryPoint.rs");
        }

        pub mod paymaster {
            include_abi!("IPaymaster.rs");
        }

        pub mod stake_manager {
            include_abi!("IStakeManager.rs");
        }
        #[allow(clippy::useless_conversion, clippy::module_inception)]
        pub mod user_operation_lib {
            include_abi!("UserOperationLib.rs");
        }
    }
}
