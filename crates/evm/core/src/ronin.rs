use alloy_primitives::{Address, Bytes, ChainId, TxKind, U256};
use serde::{Deserialize, Serialize};

pub const SPONSORED_TX_TYPE_ID: u8 = 0x64;

/// A ronin sponsored transaction.
/// See <https://github.com/ronin-chain/REPs/blob/main/REP-0008/REP-0008.md>
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxSponsored {
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity", rename = "chainId"))]
    pub chain_id: ChainId,

    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub nonce: u64,

    #[cfg_attr(
        feature = "serde",
        serde(with = "alloy_serde::quantity", rename = "maxPriorityFeePerGas")
    )]
    pub max_priority_fee_per_gas: u128,

    #[cfg_attr(
        feature = "serde",
        serde(with = "alloy_serde::quantity", rename = "maxFeePerGas")
    )]
    pub max_fee_per_gas: u128,

    #[cfg_attr(
        feature = "serde",
        serde(with = "alloy_serde::quantity", rename = "gas")
    )]
    pub gas_limit: u64,

    pub to: Option<TxKind>,

    pub value: U256,
    pub input: Bytes,

    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity", rename = "expiredTime"))]
    pub expired_time: u64,

    pub payer: Address,
}
