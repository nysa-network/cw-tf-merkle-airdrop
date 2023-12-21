use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use sha2::Digest;

#[cw_serde]
pub struct InstantiateMsg {
    pub merkle_root: String,
    pub native_token: String,
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
    fn can_validate_proof() {
        let m_root = "3a1cf2a6d59ba25658449800c9ba1a9346e3072386e6e3862a075b789b5eeeda".to_string();

        let proof: Vec<String> = vec![
            "b54aa6e2bebfb1df809051efe57d95bee5cffb7d89cafc402f8663de4af135b1".to_string(),
            "3388109033b3b507b472e280438ad3be08f205004c1b14edbb1c7dd2ede80ed9".to_string(),
            "fbbced22b298fa3de1228f15590cead614aedd07cbfe4961a8f0ce62257a4fbf".to_string(),
        ];

        let claim_msg = ClaimMsg {
            proof,
            claimer_addr: "inj1q08vl6nwcqe9hm29pg6ral02uh45rnakcuhajf".to_string(),
            amount: Uint128::new(100000000),
        };

        assert!(claim_msg.validate_proof(m_root));
    }
}
