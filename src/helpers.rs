use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg, DepsMut, Env, HexBinary, StdError, Response};

use crate::msg::ExecuteMsg;
use crate::state::Campaign;

use cosmwasm_std::instantiate2_address;

use crate::error::ContractError;

use crate::state::CAMPAIGNS;


pub fn create_campaign(
    deps: DepsMut,
    env: Env,
    chain_id: String, 
    validator_addr: String, 
    target_pos: u8
) -> Result<Response, ContractError> {
    // deploy campaign contract


    // create and return campaign 
    let canonical_creator = deps.api.addr_canonicalize(env.contract.address.as_str())?;
    let checksum = HexBinary::from_hex("9af782a3a1bcbcd22dbb6a45c751551d9af782a3a1bcbcd22dbb6a45c751551d")?;
    let salt = b"instance 1231";
    let campaign_addr = instantiate2_address(&checksum, &canonical_creator, salt)
        .map_err(|_| StdError::generic_err("Could not calculate addr"))?;
    let mut campaign = Campaign{ campaign_addr: campaign_addr.to_string(), target_pos, active: true };

    CAMPAIGNS.save(deps.storage, (chain_id, validator_addr), &campaign);

    Ok(Response::new())
}


/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}
