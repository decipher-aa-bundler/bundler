use crate::ethereum::errors::EthereumError;
use contracts::bindings::abi::entry_point;
use ethers::prelude::state;
use ethers::types::{Bytes, U256};
use ethers::utils::Units::Ether;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait ValidateSimulation {
    fn validate(&self) -> Result<(), EthereumError>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct ReturnInfo {
    pub pre_op_gas: U256,
    pub prefund: U256,
    pub sig_failed: bool,
    pub valid_after: u64,
    pub valid_until: u64,
    pub paymaster_context: Bytes,
}

impl ValidateSimulation for ReturnInfo {
    fn validate(&self) -> Result<(), EthereumError> {
        let now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| -> EthereumError::ValidateError { reason: e })
            .as_secs();

        if self.sig_failed {
            return Err(EthereumError::ValidateError {
                reason: String::from("sig failed"),
            });
        }
        if now < self.valid_after {
            return Err(EthereumError::ValidateError {
                reason: String::from("too early to get allowance"),
            });
        }

        if now > self.valid_until {
            return Err(EthereumError::ValidateError {
                reason: String::from("allowance expired"),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SenderInfo {
    pub stake: U256,
    pub unstake_delay_sec: U256,
}

impl ValidateSimulation for SenderInfo {
    fn validate(&self) -> Result<(), EthereumError> {
        if self.stake < U256::exp10(18) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake amount"),
            });
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake time"),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FactoryInfo {
    pub stake: U256,
    pub unstake_delay_sec: U256,
}

impl ValidateSimulation for FactoryInfo {
    fn validate(&self) -> Result<(), EthereumError> {
        if self.stake < U256::exp10(18) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake amount"),
            });
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake time"),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PaymasterInfo {
    pub stake: U256,
    pub unstake_delay_sec: U256,
}

impl ValidateSimulation for PaymasterInfo {
    fn validate(&self) -> Result<(), EthereumError> {
        if self.stake < U256::exp10(18) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake amount"),
            });
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake time"),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AggregatorInfo {
    pub stake: U256,
    pub unstake_delay_sec: U256,
}

impl ValidateSimulation for AggregatorInfo {
    fn validate(&self) -> Result<(), EthereumError> {
        if self.stake < U256::exp10(18) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake amount"),
            });
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError {
                reason: String::from("insufficient stake time"),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidationResult {
    pub return_info: ReturnInfo,
    pub sender_info: SenderInfo,
    pub factory_info: FactoryInfo,
    pub paymaster_info: PaymasterInfo,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidationResultWithAggregator {
    pub return_info: ReturnInfo,
    pub sender_info: SenderInfo,
    pub factory_info: FactoryInfo,
    pub paymaster_info: PaymasterInfo,
    pub aggregator_info: AggregatorInfo,
}

impl From<ValidationResult> for entry_point::ValidationResult {
    fn from(value: ValidationResult) -> Self {
        entry_point::ValidationResult {
            return_info: (
                value.return_info.pre_op_gas,
                value.return_info.prefund,
                value.return_info.sig_failed,
                value.return_info.valid_after,
                value.return_info.valid_until,
                value.return_info.paymaster_context,
            ),
            sender_info: (value.sender_info.stake, value.sender_info.unstake_delay_sec),
            factory_info: (
                value.factory_info.stake,
                value.factory_info.unstake_delay_sec,
            ),
            paymaster_info: (
                value.paymaster_info.stake,
                value.paymaster_info.unstake_delay_sec,
            ),
        }
    }
}

impl From<ValidationResultWithAggregator> for entry_point::ValidationResultWithAggregation {
    fn from(value: ValidationResultWithAggregator) -> Self {
        entry_point::ValidationResultWithAggregator {
            return_info: (
                value.return_info.pre_op_gas,
                value.return_info.prefund,
                value.return_info.sig_failed,
                value.return_info.valid_after,
                value.return_info.valid_until,
                value.return_info.paymaster_context,
            ),
            sender_info: (value.sender_info.stake, value.sender_info.unstake_delay_sec),
            factory_info: (
                value.factory_info.stake,
                value.factory_info.unstake_delay_sec,
            ),
            paymaster_info: (
                value.paymaster_info.stake,
                value.paymaster_info.unstake_delay_sec,
            ),
            aggregator_info: (
                value.aggregator_info.stake,
                value.aggregator_info.unstake_delay_sec,
            ),
        }
    }
}

impl ValidateSimulation for ValidationResult {
    fn validate(&self) -> Result<(), EthereumError> {
        self.factory_info.validate()?;
        self.paymaster_info.validate()?;
        self.sender_info.validate()?;
        self.return_info.validate()?;

        Ok(())
    }
}

impl ValidateSimulation for ValidationResultWithAggregator {
    fn validate(&self) -> Result<(), EthereumError> {
        self.factory_info.validate()?;
        self.paymaster_info.validate()?;
        self.sender_info.validate()?;
        self.return_info.validate()?;
        self.aggregator_info.validate()?;

        Ok(())
    }
}
