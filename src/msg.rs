use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use sha2::Digest;

#[cw_serde]
pub struct InstantiateMsg {
    pub merkle_root: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Claim(ClaimMsg),
}

#[cw_serde]
pub struct ClaimMsg {
    pub proof: Vec<String>,
    pub claimer_addr: String,
    pub amount: Uint128,
}

impl ClaimMsg {
    fn user_input(&self) -> String {
        format!("{}:{}", self.claimer_addr, self.amount)
    }

    pub fn validate_proof(&self, merkle_root: String) -> bool {
        let merkle_root = hex::decode(merkle_root).unwrap();

        let hash = sha2::Sha256::digest(self.user_input().as_bytes())
            .as_slice()
            .try_into()
            .unwrap();

        let hash = self
            .proof
            .clone()
            .into_iter()
            .try_fold(hash, |hash, p| {
                let mut proof_buf = [0; 32];
                hex::decode_to_slice(p, &mut proof_buf).unwrap();
                let mut hashes = [hash, proof_buf];
                hashes.sort_unstable();
                sha2::Sha256::digest(&hashes.concat()).as_slice().try_into()
            })
            .unwrap();

        merkle_root == hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_proof() {
        let m_root = "cbb6637cb3044364f16ddeddfae53efbaeeb08cf76ba364c9d8f8af7081dd855".to_string();

        let proof: Vec<String> = vec![
            "8bd1a3b12cb2fd35eda1fd59edb390045a85e88304d4d10a4494907627cff5e5".to_string(),
            "ff245e84595d53a3356fddcf73f177b130670f26b82206b1040f0c4de07aa8ea".to_string(),
        ];

        let claim_msg = ClaimMsg {
            proof,
            claimer_addr: "inj1yyy".to_string(),
            amount: Uint128::new(10),
        };

        assert!(claim_msg.validate_proof(m_root));
    }
}
