use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Coin,
};

mod msg;
mod state;

use crate::msg::{
    InstantiateMsg, ExecuteMsg, QueryMsg, OptionType, MarketResponse, PoolResponse, UserStakeResponse,
};
use crate::state::{MARKET, USER_STAKES, MarketInfo, UserStake};

// Version information for storage
const CONTRACT_NAME: &str = "cw_prediction_market";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, String> {
    let market_info = MarketInfo {
        event_name: msg.event_name,
        description: msg.description,
        asset_address: msg.asset_address,
        deadline: msg.deadline,
        pool_yes: 0,
        pool_no: 0,
        winning_option: None,
        fee_percentage: msg.fee_percentage,
    };

    MARKET.save(deps.storage, &market_info)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.as_str()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, String> {
    match msg {
        ExecuteMsg::Stake { option, amount } => {
            // TODO: Validate asset and amount
            let mut market_info = MARKET.load(deps.storage)?;
            let mut user_stakes = USER_STAKES.load(deps.storage)?;

            match option {
                OptionType::Yes => {
                    market_info.pool_yes += amount;
                }
                OptionType::No => {
                    market_info.pool_no += amount;
                }
            }

            MARKET.save(deps.storage, &market_info)?;

            user_stakes.push(UserStake {
                option,
                amount,
            });

            USER_STAKES.save(deps.storage, &user_stakes)?;

            Ok(Response::new()
                .add_attribute("method", "stake")
                .add_attribute("option", format!("{:?}", option))
                .add_attribute("amount", amount.to_string()))
        }
        ExecuteMsg::CloseMarket { winning_option } => {
            // TODO: Check if deadline has passed
            let mut market_info = MARKET.load(deps.storage)?;
            market_info.winning_option = Some(winning_option);
            MARKET.save(deps.storage, &market_info)?;

            Ok(Response::new()
                .add_attribute("method", "close_market")
                .add_attribute("winning_option", format!("{:?}", winning_option)))
        }
        ExecuteMsg::ClaimReward {} => {
            // TODO: Implement reward claiming logic
            Ok(Response::new().add_attribute("method", "claim_reward"))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMarket {} => {
            let market_info = MARKET.load(deps.storage)?;
            to_json_binary(&MarketResponse {
                event_name: market_info.event_name,
                description: market_info.description,
                asset_address: market_info.asset_address,
                deadline: market_info.deadline,
                pool_yes: market_info.pool_yes,
                pool_no: market_info.pool_no,
                winning_option: market_info.winning_option,
                fee_percentage: market_info.fee_percentage,
            })
        }
        QueryMsg::GetPool { option } => {
            let market_info = MARKET.load(deps.storage)?;
            let amount = match option {
                OptionType::Yes => market_info.pool_yes,
                OptionType::No => market_info.pool_no,
            };
            to_json_binary(&PoolResponse { amount })
        }
        QueryMsg::GetUserStake { address, option } => {
            // TODO: Implement user stake query logic
            to_json_binary(&UserStakeResponse { amount: 0 })
        }
    }
}
