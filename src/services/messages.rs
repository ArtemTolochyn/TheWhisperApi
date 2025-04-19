use crate::models::{Message, MessageContent, MessageResponse};
use crate::utils::Crypto;
pub fn handle_remove_message_status(res: &reqwest::Response) -> Result<(), String>
{
    match res.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::NOT_FOUND => Err("Message not found".to_string()),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err("Internal server error".to_string()),
        _ => Err("Unknown error".to_string()),
    }
}

pub fn handle_send_message_status(res: &reqwest::Response) -> Result<(), String>
{
    match res.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::NOT_FOUND => Err("Channel not found".to_string()),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err("Internal server error".to_string()),
        _ => Err("Unknown error".to_string()),
    }
}

pub fn validate_messages(message_response: Vec<MessageResponse>, key: [u8; 32]) -> Result<Vec<Message>, String>
{
    let crypto = Crypto::new(key)?;
    let mut proven_messages: Vec<Message> = Vec::new();

    for message in message_response.iter()
    {
        let content = match crypto.decrypt_string(message.content.clone()) {
            Ok(data) => match serde_json::from_str::<MessageContent>(&data) {
                Ok(data) => Some(data),
                Err(_) => None
            },
            Err(_) => None
        };

        let message = Message {
            id: message.id,
            chat_id: message.chat_id,
            user_id: message.user_id,
            content,
            timestamp: message.timestamp,
        };

        proven_messages.push(message);
    };

    Ok(proven_messages)
}

pub fn encrypt_message(key: [u8; 32], text: String) -> Result<String, String>
{
    let crypto = Crypto::new(key)?;

    let message_content = MessageContent { text };
    let message_content_serialized = serde_json::to_string(&message_content)
        .map_err(|_| "Cannot serialize message content")?;

    crypto.encrypt_string(message_content_serialized)
        .map_err(|_| "Cannot encrypt message content".to_string())
}