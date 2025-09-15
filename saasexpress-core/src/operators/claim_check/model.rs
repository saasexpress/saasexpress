use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ClaimCheckReference {
    pub claim_type: String,
    pub claim_value: String,
}
