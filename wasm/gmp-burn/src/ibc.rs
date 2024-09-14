use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin};

#[cw_serde]
pub struct IbcCounterpartyHeight {
    pub revision_number: Option<u64>,
    pub revision_height: Option<u64>,
}

#[cw_serde]
pub struct MsgTransfer {
    pub source_port: String,
    pub source_channel: String,
    pub token: Option<Coin>,
    pub sender: String,
    pub receiver: String,
    pub timeout_height: Option<IbcCounterpartyHeight>,
    pub timeout_timestamp: Option<u64>,
    pub memo: String,
}

impl MsgTransfer {
    pub fn into_binary(self) -> Binary {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(b"{\"transfer\":");
        encoded.extend_from_slice(&serde_json_wasm::to_vec(&self).unwrap());
        encoded.push(b'}');
        Binary(encoded)
    }
}