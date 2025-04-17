use crate::models::{LoginChallengeResponse, LoginTokenResponse};

pub fn handle_status(res: &reqwest::Response) -> Result<(), String>
{
    match res.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::BAD_REQUEST => Err("Bad request".to_string()),
        reqwest::StatusCode::CONFLICT => Err("User already exists".to_string()),
        reqwest::StatusCode::NOT_FOUND => Err("User not found".to_string()),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err("Internal server error".to_string()),
        _ => Err("Unknown error".to_string()),
    }
}

pub fn parse_challenge(json_string: String) -> Result<String, String>
{
    let json_parsed: LoginChallengeResponse = serde_json::from_str(&json_string)
        .map_err(|_| "Cannot parse json".to_string())?;

    Ok(json_parsed.challenge)
}

pub fn parse_token(json_string: String) -> Result<String, String>
{
    let json_parsed: LoginTokenResponse = serde_json::from_str(&json_string).map_err(|_| "Cannot parse json".to_string())?;
    Ok(json_parsed.token)
}