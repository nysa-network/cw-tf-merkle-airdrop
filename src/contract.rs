#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ClaimMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CLAIMED_ADDRESSES, CONFIG};

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
        native_token: msg.native_token,
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
    info: MessageInfo,
    claim_msg: ClaimMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage).unwrap();

    let claimed = CLAIMED_ADDRESSES.may_load(deps.storage, claim_msg.claimer_addr.as_str())?;
    if claimed.is_some() {
        return Err(ContractError::Claimed {});
    }

    if !claim_msg.validate_proof(config.merkle_root) {
        return Err(ContractError::InvalidProof {});
    }

    CLAIMED_ADDRESSES.save(deps.storage, claim_msg.claimer_addr.as_str(), &true)?;

    let msg = BankMsg::Send {
        to_address: claim_msg.claimer_addr,
        amount: vec![Coin {
            denom: config.native_token,
            amount: claim_msg.amount,
        }],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "claim")
        .add_attribute("address", info.sender.to_string())
        .add_attribute("amount", claim_msg.amount)
        .add_attribute("action", "claim"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HasClaimed { claimer_addr } => query_has_claimed(deps, claimer_addr),
    }
}

pub fn query_has_claimed(deps: Deps, addr: String) -> StdResult<Binary> {
    let claimed = CLAIMED_ADDRESSES.may_load(deps.storage, addr.as_str())?;

    to_json_binary(&claimed.is_some())
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
    fn cain_claim_native_token() {
        let merkle_root =
            "3a1cf2a6d59ba25658449800c9ba1a9346e3072386e6e3862a075b789b5eeeda".to_string();
        let mut deps = mock_dependencies();
        let info = mock_info(
            &"admin1".to_string(),
            &coins(21_000_000, "factory/inj1admin/utoken"),
        );

        let proof: Vec<String> = vec![
            "b54aa6e2bebfb1df809051efe57d95bee5cffb7d89cafc402f8663de4af135b1".to_string(),
            "3388109033b3b507b472e280438ad3be08f205004c1b14edbb1c7dd2ede80ed9".to_string(),
            "fbbced22b298fa3de1228f15590cead614aedd07cbfe4961a8f0ce62257a4fbf".to_string(),
        ];

        let claim_msg = ClaimMsg {
            proof: proof.clone(),
            claimer_addr: "inj1q08vl6nwcqe9hm29pg6ral02uh45rnakcuhajf".to_string(),
            amount: Uint128::new(100000000),
        };

        // Instantiate
        let instantiate_msg = InstantiateMsg {
            merkle_root: merkle_root,
            native_token: "factory/inj1adminx/utoken".to_string(),
        };
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Claim
        let info = mock_info(&"Alice".to_string(), &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Claim(claim_msg.clone()),
        )
        .unwrap();
        assert_eq!(1, res.messages.len());

        // println!("res: {:?}", res);
    }
}
