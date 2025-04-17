use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RegisterRequest {
    pub(crate) username: String,
    pub(crate) public_key: String
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub(crate) username: String
}

#[derive(Serialize)]
pub struct LoginVerifyRequest {
    pub(crate) username: String,
    pub(crate) response: String
}

#[derive(Deserialize)]
pub struct LoginChallengeResponse {
    pub(crate) challenge: String
}

#[derive(Deserialize)]
pub struct LoginTokenResponse {
    pub(crate) token: String
}
