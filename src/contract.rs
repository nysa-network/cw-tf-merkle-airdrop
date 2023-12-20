#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ClaimMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-tf-merkle-airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cfg = Config {
        merkle_root: msg.merkle_root,
        owner: info.sender.clone(),
    };
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Claim(msg) => execute_claim(deps, info, msg),
    }
}

pub fn execute_claim(
    deps: DepsMut,
    _info: MessageInfo,
    msg: ClaimMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage).unwrap();

    if !msg.validate_proof(config.merkle_root) {
        return Err(ContractError::InvalidProof {});
    }

    Ok(Response::new().add_attribute("action", "claim"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

// TESTS
// ==============================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::ExecuteMsg;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Uint128};

    #[test]
    fn test_claim() {
        let merkle_root =
            "cbb6637cb3044364f16ddeddfae53efbaeeb08cf76ba364c9d8f8af7081dd855".to_string();
        let mut deps = mock_dependencies();
        let info = mock_info(
            &"ADMIN".to_string(),
            &coins(21_000_000, "factory/inj1admin/utoken"),
        );

        let proof: Vec<String> = vec![
            "8bd1a3b12cb2fd35eda1fd59edb390045a85e88304d4d10a4494907627cff5e5".to_string(),
            "ff245e84595d53a3356fddcf73f177b130670f26b82206b1040f0c4de07aa8ea".to_string(),
        ];

        let claim_msg = ClaimMsg {
            proof,
            claimer_addr: "inj1yyy".to_string(),
            amount: Uint128::new(10),
        };

        // Instantiate
        let instantiate_msg = InstantiateMsg {
            merkle_root: merkle_root,
        };
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Claim
        let info = mock_info(&"Alice".to_string(), &coins(10, "factory/inj1admin/utoken"));
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Claim(claim_msg),
        )
        .unwrap();
        assert_eq!(0, res.messages.len());
    }
}
