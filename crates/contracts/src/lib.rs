pub mod ethereum {
    pub mod entry_point {
        include!("ethereum/IEntryPoint.rs");
    }
    pub mod aggregator {
        include!("ethereum/IAggregator.rs");
    }
}