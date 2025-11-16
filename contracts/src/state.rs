use cw_storage_plus::Item;

pub const MARKET: Item<MarketInfo> = Item::new("market");
pub const USER_STAKES: Item<Vec<UserStake>> = Item::new("user_stakes");

#[cw_serde]
pub struct MarketInfo {
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
pub struct UserStake {
    pub option: OptionType,
    pub amount: u128,
}
