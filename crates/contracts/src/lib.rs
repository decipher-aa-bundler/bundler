#[macro_export]
macro_rules! include_abi {
    ($path:literal) => {
        include!(concat!("ethereum/", $path));
    };
}

pub mod ethereum {
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
    }
}
