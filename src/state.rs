use cw_storage_plus::Map;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Campaign {
    pub campaign_addr: String,
    pub target_pos: u8,
    pub active: bool,
}



pub const CAMPAIGNS: Map<(String, String), Campaign> = Map::new("campaigns");

pub const CODE_ID: u8 = 100; ///////////