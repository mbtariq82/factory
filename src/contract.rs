#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::CAMPAIGNS;

use crate::helpers::create_campaign;
use crate::state::Campaign;

use std::collections::HashMap;




/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) {
    //let mut campaigns: HashMap<(String, String), Campaign> = HashMap::new();
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateCampaign{chain_id, validator_addr, target_pos} => create_campaign(
            deps,
            env,
            chain_id, 
            validator_addr, 
            target_pos,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCampaign{chain_id, validator_addr} => {
            unimplemented!();
        }
        QueryMsg::GetAllCampaigns{} => {
            unimplemented!();
        }
    }
}

#[cfg(test)]
mod tests {}
