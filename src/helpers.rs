use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg, DepsMut, Env, HexBinary, StdError, Response, instantiate2_address};

use crate::msg::ExecuteMsg;

use crate::error::ContractError;

use crate::state::{Campaign, CAMPAIGNS, CODE_ID};

use sha2::{Sha256, Digest};



pub fn create_campaign(
    deps: DepsMut,
    env: Env,
    chain_id: String, 
    validator_addr: String, 
    target_pos: u8
) -> Result<Response, ContractError> {
    let canonical_creator = deps.api.addr_canonicalize(env.contract.address.as_str())?;

    // Convert u8 CODE_ID to a string and calculate the checksum
    let code_id_str = CODE_ID.to_string();
    let mut hasher = Sha256::new();
    hasher.update(code_id_str.as_bytes());
    let checksum = hasher.finalize();
    
    // Convert checksum to Vec<u8>
    let checksum_bytes: Vec<u8> = checksum.iter().cloned().collect();

    let salt = b"instance 1231";
    let campaign_addr = instantiate2_address(&checksum, &canonical_creator, salt)
        .map_err(|_| StdError::generic_err("Could not calculate addr"))?;
    let mut campaign = Campaign{ campaign_addr: campaign_addr.to_string(), target_pos, active: true };

    CAMPAIGNS.save(deps.storage, (chain_id, validator_addr), &campaign);

    Ok(Response::new())
}

pub fn get_campaign(
    deps: DepsMut,
    chain_id: String, 
    validator_addr: String,
) -> Result<Response, ContractError> {
    let key = (chain_id.clone(), validator_addr.clone());
    let campaign: Campaign = CAMPAIGNS.load(deps.storage, key)?;

    // Handle the retrieved campaign data here
    let response_msg = format!(
        "Campaign found: Address={}, Target Pos={}, Active={}",
        campaign.campaign_addr, campaign.target_pos, campaign.active
    );

    Ok(Response::new().add_attributes(vec![("campaign_info", response_msg)]))
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
