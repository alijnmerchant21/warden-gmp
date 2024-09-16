#[cfg(not(feature = "library"))]
use cosmwasm_std::{Uint256, DepsMut, Env, MessageInfo, Response};
use ethabi::{encode, Token};
use serde_json_wasm::to_string;
//use ethabi::ethereum_types::U256;

// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::*;
// use crate::state::*;

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
    match msg {
        ExecuteMsg::SendMessageEvm { amount_to_burn } => {
            exec::send_message_evm(deps, env, info, amount_to_burn)
        }

    }
}

mod exec {
    use super::*;
    use ethabi::ethereum_types::U256;

    fn create_burn_payload(amount: Uint256) -> Result<Vec<u8>, ContractError> {
        // Convert Uint256 to U256 using its byte representation
        let amount_bytes = amount.to_be_bytes(); // Uint256 to big-endian bytes
        let amount_u256 = U256::from_big_endian(&amount_bytes); // Convert bytes to ethabi::U256
        
        let amount_token = Token::Uint(amount_u256);
        Ok(encode(&[amount_token]))
    }

    // Sends a message via Axelar GMP to the EVM {destination_chain} and {destination_address}
    pub fn send_message_evm(
        _deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount_to_burn: Uint256,  // Amount to burn
    ) -> Result<Response, ContractError> {
        
        // Hardcode the destination values
        let destination_chain = "ethereum-sepolia".to_string();
        let destination_address = "0x5388dE880a16Ba9602746F3799E850E78148cd57".to_string();

        // Create the payload
        let payload = create_burn_payload(amount_to_burn)?;

        // {info.funds} used to pay gas. Must only contain 1 token type
        let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

    
        // Message payload to be received by the destination
        /*let message_payload = encode(&vec![
            Token::String(info.sender.to_string()),  // Sender info
            Token::String(message),  // The message itself
        ]);*/

    
        let gmp_message: GmpMessage = GmpMessage {
            destination_chain,
            destination_address,
            payload,
            type_: 1,
            fee: Some(Fee {
                amount: coin.amount.to_string(),
                recipient: "axelar1zl3rxpp70lmte2xr6c4lgske2fyuj3hupcsvcd".to_string(),
            }),
        };
    
        let ibc_message = crate::ibc::MsgTransfer {
            source_port: "transfer".to_string(),
            source_channel: "channel-1".to_string(), // Warden Testnet
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