use crate::models::{Channel, ChannelResponse};
use crate::utils::{rsa_api, Crypto};

pub fn handle_status(res: &reqwest::Response) -> Result<(), String>
{
    match res.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::BAD_REQUEST => Err("Bad request".to_string()),
        reqwest::StatusCode::CONFLICT => Err("Channel already exists".to_string()),
        reqwest::StatusCode::NOT_FOUND => Err("Channel not found".to_string()),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err("Internal server error".to_string()),
        _ => Err("Unknown error".to_string()),
    }
}

pub fn handle_status_join(res: &reqwest::Response) -> Result<(), String>
{
    match res.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::BAD_REQUEST => Err("Bad request".to_string()),
        reqwest::StatusCode::CONFLICT => Err("You already in the channel".to_string()),
        reqwest::StatusCode::NOT_FOUND => Err("Channel not found".to_string()),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err("Internal server error".to_string()),
        _ => Err("Unknown error".to_string()),
    }
}

pub fn validate_channels(channel_response: Vec<ChannelResponse>, private_key: &str) -> Vec<Channel>
{
    let mut proven_channels: Vec<Channel> = Vec::new();

    for channel in channel_response.iter()
    {
        let result = rsa_api::check_signature(channel.key.clone(), private_key, &channel.signature);

        let key = match result {
            Ok(key) => Some(key),
            Err(_) => None,
        };


        let channel_name = match key {
            Some(key) => {
                let crypto = Crypto::new(key);

                match crypto {
                    Ok(crypto) => {
                        crypto.decrypt_string(channel.name.clone()).unwrap_or(channel.name.clone())
                    }
                    Err(_) => channel.name.clone()
                }
            }
            None => channel.name.clone()
        };

        let channel = Channel {
            id: channel.id,
            name: channel_name,
            last_edited: channel.last_edited,
            key,
        };

        proven_channels.push(channel);
    }

    proven_channels
}