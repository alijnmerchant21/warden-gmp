// SPDX-License-Identifier: MIT
use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, SubMsg, WasmMsg, to_json_binary,
};
use ethabi::{encode, Token};
use serde::{Deserialize, Serialize};
//use serde_json_wasm::to_string;
use cw_utils;
use crate::ibc::MsgTransfer;

use crate::msg::{ExecuteMsg, InstantiateMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GmpMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    pub message_type: u8,
    pub fee: Option<String>,
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // No initialization logic required
    Ok(Response::default())
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
   //_deps: DepsMut,
    env: Env,
    info: MessageInfo,
    //destination_chain: String,
    //destination_address: String,
    amount: String,
) -> Result<Response, StdError> {

    // Hardcoded destination chain and address
    let destination_chain = "sepolia".to_string();
    let destination_address = "0x5388dE880a16Ba9602746F3799E850E78148cd57".to_string();

    // Parse the amount string to a u128
    let amount_u128 = amount.parse::<u128>().map_err(|_| StdError::generic_err("Invalid amount"))?;

    // Encode the amount as ABI-encoded uint256
    let amount_token = Token::Uint(amount_u128.into());
    let payload = encode(&[amount_token]);

    // Construct the GMP (General Message Passing) message
    let gmp_message = GmpMessage {
        destination_chain,
        destination_address,
        payload: payload.clone(),
        message_type: 1,
        fee: None,
    };

    /*// Message payload to be received by the destination
     let message_payload = encode(&vec![
        Token::String(info.sender.to_string()),
        Token::String(message),
     ]);

    // {info.funds} used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();*/


    // Use the funds sent with the transaction to pay for gas
    let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

    // Create the IBC transfer message to Axelar
    /*let ibc_msg = MsgTransfer {
        channel_id: "channel-1".to_string(),
        to_address: "0x97837985Ec0494E7b9C71f5D3f9250188477ae14".to_string(), // Axelar Gateway address
        amount: coin,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(600)),
        //memo: to_string(&gmp_message).unwrap(),
    };

    Ok(Response::new().add_message(ibc_msg))*/

    // Create the IBC transfer message
    let ibc_message = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-1".to_string(),
        token: Some(coin),
        sender: env.contract.address.to_string(),
        receiver: "0x97837985Ec0494E7b9C71f5D3f9250188477ae14".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(600).nanos()),
        memo: serde_json_wasm::to_string(&gmp_message).map_err(|_| StdError::generic_err("Failed to serialize GMP message"))?,
    };

    // Create a SubMsg to send the IBC transfer
    let sub_msg = SubMsg::new(WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_json_binary(&ibc_message.into_binary())?,
        funds: vec![],
    });

    Ok(Response::new().add_submessage(sub_msg))




}
