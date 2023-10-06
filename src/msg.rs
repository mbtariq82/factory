use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::Campaign;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateCampaign {
        chain_id: String,
        validator_addr: String,
        target_pos: u8,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Campaign)]
    GetCampaign {
        chain_id: String,
        validator_addr: String,
    },
    #[returns(Vec<Campaign>)]
    GetAllCampaigns {}
}
