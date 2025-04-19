pub mod models;
mod utils;
mod services;
mod tests;

use std::option::Option;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Response;
use serde::Serialize;
use crate::models::{Channel, CreateChannelRequest, CreateChannelResponse, GetChannelListRequest, GetChannelListResponse, GetMessagesRequest, GetMessagesResponse, JoinChannelRequest, LeaveChannelRequest, LoginRequest, LoginVerifyRequest, Message, RegisterRequest, RemoveChannelRequest, RemoveMessageRequest, SendMessageRequest, SendMessageResponse};
use crate::services::{auth, channels, messages};
use crate::utils::{rsa_api, Crypto};

pub struct WhisperClient
{
    server: String,
    token: String,
    private_key: String
}

impl WhisperClient {
    pub async fn register(server: String, username: String, private_key: String) -> Result<WhisperClient, String> {
        let public_key_base64 = rsa_api::get_public_key_base64(&private_key)?;

        let request = RegisterRequest {
            username: username.clone(),
            public_key: public_key_base64
        };

        let client = reqwest::Client::new();
        let res = client.post(
            server.clone() + "/register")
            .timeout(Duration::from_secs(5))
            .json(&request).send().await
            .map_err(|_| "Cannot connect to the server")?;


        auth::handle_status(&res)?;

        WhisperClient::login(server, username, private_key).await
    }

    pub async fn login(server: String, username: String, private_key: String) -> Result<WhisperClient, String>
    {
        let request = LoginRequest {
            username: username.clone(),
        };

        let client = reqwest::Client::new();
        let res = client.post(
            server.clone() + "/login/request")
            .timeout(Duration::from_secs(5))
            .json(&request).send().await
            .map_err(|_| "Cannot connect to the server")?;

        auth::handle_status(&res)?;

        let json_string = res.text().await.map_err(|_| "Cannot parse challenge")?;
        let challenge = auth::parse_challenge(json_string)?;
        let challenge_text = rsa_api::decrypt_string(&private_key, &challenge)?;

        let request = LoginVerifyRequest {
            username,
            response: challenge_text
        };
        let res = client.post(
            server.clone() + "/login/verify")
            .json(&request).send().await
            .map_err(|_| "Cannot connect to the server")?;

        auth::handle_status(&res)?;

        let json_string =  res.text().await.map_err(|_| "Cannot parse token")?;
        let token = auth::parse_token(json_string).map_err(|_| "Cannot parse token")?;

        Ok(
            WhisperClient {
                server,
                token,
                private_key
            })
    }

    fn get_client(&self) -> Result<reqwest::Client, String>
    {
        let mut headers = HeaderMap::new();
        let header_value = HeaderValue::from_str(&self.token)
            .map_err(|_| "Failed to create Authorization header value".to_string())?;
        headers.insert(AUTHORIZATION, header_value);

        reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|_| "Failed to build request client".to_string())
    }

    async fn send_post<T: Serialize + ?Sized>(&self, endpoint: &str, request: &T) -> Result<Response, String>
    {
        let client = self.get_client()?;
        client.post(
            self.server.clone() + endpoint)
            .json(&request).send().await
            .map_err(|_| "Cannot connect to the server".to_string())
    }


    pub async fn create_channel(&self, name: String) -> Result<Channel, String>
    {
        if name.is_empty()
        {
            return Err("Channel name is empty".to_string())
        }

        let key = rsa_api::generate_random_key();

        let crypto = Crypto::new(key)?;
        let name_encrypted = crypto.encrypt_string(name.clone())?;

        let request = CreateChannelRequest { name: name_encrypted };
        let res = self.send_post("/api/create_channel", &request).await?;

        channels::handle_status(&res)?;

        let json_string =  res.text().await.map_err(|_| "Cannot parse channel id")?;
        let json: CreateChannelResponse = serde_json::from_str(&json_string).map_err(|_| "Cannot parse channel id")?;

        self.join_channel(json.channel.id, key).await?;

        let channel = Channel {
            id: json.channel.id,
            name,
            last_edited: json.channel.last_edited,
            key: Some(key)

        };

        Ok(channel)
    }

    pub async fn join_channel(&self, id: i64, key: [u8; 32]) -> Result<(), String>
    {
        let key_encrypted = rsa_api::encrypt_bytes(&self.private_key, key.to_vec())?;
        let signature = rsa_api::generate_signature(key_encrypted.clone(), &self.private_key)?;

        let request = JoinChannelRequest { id, key: key_encrypted, signature };
        let res = self.send_post("/api/join_channel", &request).await?;

        channels::handle_status(&res)
    }

    pub async fn leave_channel(&self, id: i64) -> Result<(), String>
    {
        let request = LeaveChannelRequest { chat_id: id };
        let res = self.send_post("/api/leave_channel", &request).await?;

        channels::handle_status(&res)
    }

    pub async fn remove_channel(&self, id: i64) -> Result<(), String>
    {
        let request = RemoveChannelRequest { chat_id: id };
        let res = self.send_post("/api/remove_channel", &request).await?;

        channels::handle_status(&res)
    }

    pub async fn get_channels(&self) -> Result<Vec<Channel>, String>
    {
        let request = GetChannelListRequest;

        let res = self.send_post("/api/get_channel_list", &request).await?;

        channels::handle_status(&res)?;

        let json_string =  res.text().await.map_err(|_| "Cannot parse channel list")?;
        let json: GetChannelListResponse = serde_json::from_str(&json_string).map_err(|_| "Cannot parse channel list")?;

        Ok(channels::validate_channels(json.channels, &self.private_key))
    }

    pub async fn send_message(&self, chat_id: i64, text: String, key: [u8; 32]) -> Result<i64, String>
    {
        let content_encrypted = messages::encrypt_message(key, text)?;
        let request = SendMessageRequest {
            chat_id, content: content_encrypted
        };

        let res = self.send_post("/api/send_message", &request).await?;

        messages::handle_send_message_status(&res)?;

        let json_string =  res.text().await.map_err(|_| "Cannot parse message id")?;
        let json: SendMessageResponse = serde_json::from_str(&json_string).map_err(|_| "Cannot parse message id")?;

        Ok(json.message_id)
    }

    pub async fn remove_message(&self, message_id: i64) -> Result<(), String>
    {
        let request = RemoveMessageRequest {
            message_id
        };

        let res = self.send_post("/api/remove_message", &request).await?;

        messages::handle_remove_message_status(&res)
    }

    pub async fn get_messages(&self, chat_id: i64, message_id: Option<i64>, key: [u8; 32]) -> Result<Vec<Message>, String>
    {
        let request = GetMessagesRequest {
            chat_id,
            message_id
        };

        let res = self.send_post("/api/get_messages", &request).await?;

        channels::handle_status(&res)?;

        let json_string =  res.text().await.map_err(|_| "Cannot parse messages list")?;
        let json: GetMessagesResponse = serde_json::from_str(&json_string).map_err(|_| "Cannot parse messages list")?;

        let validate = messages::validate_messages(json.messages, key)?;

        Ok(validate)
    }
}