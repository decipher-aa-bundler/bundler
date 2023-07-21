use crate::ethereum::errors::EthereumError;
use contracts::bindings::abi::entry_point;
use ethers::types::{Address, Bytes, U256};
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
            .map_err(|e| EthereumError::ProviderError(e.to_string()))?
            .as_secs();

        if self.sig_failed {
            return Err(EthereumError::ValidateError("sig failed".to_string()));
        }
        if now < self.valid_after {
            return Err(EthereumError::ValidateError("No allowance".to_string()));
        }

        if now > self.valid_until {
            return Err(EthereumError::ValidateError("No allowance".to_string()));
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
            return Err(EthereumError::ValidateError(String::from(
                "senderInfo: insufficient stake amount",
            )));
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError(String::from(
                "senderInfo: insufficient stake time",
            )));
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
            return Err(EthereumError::ValidateError(String::from(
                "factoryInfo: insufficient stake amount",
            )));
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError(String::from(
                "factoryInfo: insufficient stake time",
            )));
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
            return Err(EthereumError::ValidateError(String::from(
                "PaymasterInfo: insufficient stake amount",
            )));
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError(String::from(
                "PaymasterInfo: insufficient stake time",
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AggregatorInfo {
    pub address: Address,
    pub stake: U256,
    pub unstake_delay_sec: U256,
}

impl ValidateSimulation for AggregatorInfo {
    fn validate(&self) -> Result<(), EthereumError> {
        if self.stake < U256::exp10(18) {
            return Err(EthereumError::ValidateError(String::from(
                "AggregatorInfo: insufficient stake amount",
            )));
        }

        if self.unstake_delay_sec < U256::from(1) {
            return Err(EthereumError::ValidateError(String::from(
                "AggregatorInfo: insufficient stake time",
            )));
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

impl From<entry_point::ValidationResult> for ValidationResult {
    fn from(value: entry_point::ValidationResult) -> Self {
        ValidationResult {
            return_info: ReturnInfo {
                pre_op_gas: value.return_info.0,
                prefund: value.return_info.1,
                sig_failed: value.return_info.2,
                valid_after: value.return_info.3,
                valid_until: value.return_info.4,
                paymaster_context: value.return_info.5,
            },
            sender_info: SenderInfo {
                stake: value.sender_info.0,
                unstake_delay_sec: value.sender_info.1,
            },
            factory_info: FactoryInfo {
                stake: value.factory_info.0,
                unstake_delay_sec: value.factory_info.1,
            },
            paymaster_info: PaymasterInfo {
                stake: value.paymaster_info.0,
                unstake_delay_sec: value.paymaster_info.1,
            },
        }
    }
}

impl From<ValidationResultWithAggregator> for entry_point::ValidationResultWithAggregation {
    fn from(value: ValidationResultWithAggregator) -> Self {
        entry_point::ValidationResultWithAggregation {
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
                value.aggregator_info.address,
                (
                    value.aggregator_info.stake,
                    value.aggregator_info.unstake_delay_sec,
                ),
            ),
        }
    }
}

impl ValidateSimulation for ValidationResult {
    fn validate(&self) -> Result<(), EthereumError> {
        // self.factory_info.validate()?;
        // self.paymaster_info.validate()?;
        // self.sender_info.validate()?;

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
