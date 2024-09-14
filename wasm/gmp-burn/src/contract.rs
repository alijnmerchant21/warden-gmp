#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ethabi::{decode, encode, ParamType, Token};
use serde_json_wasm::to_string;

// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;



pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}



pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        SendMessageEvm {
            destination_chain,
            destination_address,
            message,
        } => exec::send_message_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        )

    }
}


mod exec {
    use super::*;

    // Sends a message via Axelar GMP to the EVM {destination_chain} and {destination_address}
    pub fn send_message_evm(
        _deps: DepsMut,
        env: Env,
        info: MessageInfo,
        destination_chain: String,
        destination_address: String,
        message: String,
    ) -> Result<Response, ContractError> {
        // Message payload to be received by the destination
        let message_payload = encode(&vec![
            Token::String(info.sender.to_string()),
            Token::String(message),
        ]);

        // {info.funds} used to pay gas. Must only contain 1 token type.
        let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

        let gmp_message: GmpMessage = GmpMessage {
            destination_chain,
            destination_address,
            payload: message_payload.to_vec(),
            type_: 1,
            fee: None,
        };

        let ibc_message = crate::ibc::MsgTransfer {
            source_port: "transfer".to_string(),
            source_channel: "channel-3".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
            token: Some(coin.into()),
            sender: env.contract.address.to_string(),
            receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
                .to_string(),
            timeout_height: None,
            timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
            memo: to_string(&gmp_message).unwrap(),
        };

        Ok(Response::new().add_message(ibc_message))
    }
}

/* SPDX-License-Identifier: MIT

//use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response};
use ethabi::{encode, Token};
use serde_json_wasm::to_string;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContract;


use crate::error::ContractError;
use crate::msg::*;
use crate::state::{Config, CONFIG};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let cfg = Config {
        channel: msg.channel,
    };
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendMessageEvm {
            destination_chain,
            destination_address,
            message,
        } => send_message_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
    }
}


pub fn send_message_evm(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Hardcode the destination values
    let destination_chain = "sepolia".to_string();
    let destination_address ="0x5388dE880a16Ba9602746F3799E850E78148cd57".to_string();

    // Convert the string 'message' into a uint256 (amount)
    
    let amount: u128 = message.parse().map_err(|_| ContractError::InvalidInput(message.clone()))?;


    // Create the message payload using EVM-compatible encoding
    let message_payload = encode(&vec![
        Token::String(info.sender.to_string()),
        Token::Uint(amount.into()),
    ]);

    // Extract the coin used to pay for gas (must be only one type)
    let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

    // Set up the fee structure
    let fee: Option<Fee> = Some(Fee {
        amount: coin.amount.to_string(),
        recipient: String::from("axelar1zl3rxpp70lmte2xr6c4lgske2fyuj3hupcsvcd"),
    });

    // Create the GMP message
    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: 1,
        fee,
    };

    // Construct the IBC transfer message to Axelar network
    let ibc_message = crate::ibc::MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.channel.to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzuj3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&gmp_message).unwrap(),
    };

    let cosmos_msg: CosmosMsg = MsgExecuteContract {
        sender: env.contract.address.to_string(),
        contract: env.contract.address.to_string(),
        msg: serde_json_wasm::to_string(&ibc_message)?,
        funds: vec![],
    }
    .into();

    Ok(Response::new().add_message(cosmos_msg))
}







use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, SubMsg, WasmMsg, to_json_binary,
};
use ethabi::{encode, Token};
use serde::{Deserialize, Serialize};
//use serde_json_wasm::to_string;
use cw_utils;
use crate::ibc::MsgTransfer;
//use crate::state::Config;
use crate::state::CONFIG;

use crate::msg::{ExecuteMsg, InstantiateMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GmpMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    pub message_type: u8,
    pub fee: Option<String>,
    pub gas_limit: Option<u64>,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // No initialization logic required
    let config = Config {
        channel: msg.channel,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]

pub fn execute(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::SendMessageEvm { amount, .. } => send_message_evm(env, info, amount),
    }
}

fn send_message_evm(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: String,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    // Parse the amount string to a u128
    let amount_u128 = amount.parse::<u128>().map_err(|_| StdError::generic_err("Invalid amount"))?;

    // Encode the amount as ABI-encoded uint256
    let amount_token = Token::Uint(amount_u128.into());
    let payload = encode(&[amount_token]);

    // Construct the GMP (General Message Passing) message
    let gmp_message = GmpMessage {
        destination_chain: "sepolia".to_string(),
        destination_address: "0x5388dE880a16Ba9602746F3799E850E78148cd57".to_string(), // EVM contract address
        payload: payload.clone(),
        message_type: 1,
        fee: None,
    };

    // Create the IBC transfer message
    let ibc_message = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.channel.clone(),
        token: None, // We're not sending any tokens
        sender: env.contract.address.to_string(),
        receiver: "0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6".to_string(), // Gas Service Contract
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(600).nanos()),
        memo: serde_json_wasm::to_string(&gmp_message).map_err(|_| StdError::generic_err("Failed to serialize GMP message"))?,
    };

    // Create a SubMsg to send the IBC transfer
    let sub_msg = SubMsg::new(WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_json_binary(&ibc_message.into_binary())?,
        funds: vec![], // No funds are being sent
    });

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "send_message_evm")
        .add_attribute("amount", amount))
}*/