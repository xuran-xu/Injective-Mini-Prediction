use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub event_name: String,
    pub description: String,
    pub asset_address: String,
    pub deadline: u64,
    pub fee_percentage: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    Stake {
        option: OptionType,
        amount: u128,
    },
    CloseMarket {
        winning_option: OptionType,
    },
    ClaimReward {},
}

#[cw_serde]
pub enum QueryMsg {
    GetMarket {},
    GetPool {
        option: OptionType,
    },
    GetUserStake {
        address: String,
        option: OptionType,
    },
}

#[cw_serde]
pub enum OptionType {
    Yes,
    No,
}

#[cw_serde]
pub struct MarketResponse {
    pub event_name: String,
    pub description: String,
    pub asset_address: String,
    pub deadline: u64,
    pub pool_yes: u128,
    pub pool_no: u128,
    pub winning_option: Option<OptionType>,
    pub fee_percentage: u8,
}

#[cw_serde]
pub struct PoolResponse {
    pub amount: u128,
}

#[cw_serde]
pub struct UserStakeResponse {
    pub amount: u128,
}
