use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MessageResponse {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub content: Option<MessageContent>,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageContent {
    pub text: String
}

#[derive(Serialize)]
pub struct SendMessageRequest
{
    pub(crate) chat_id: i64,
    pub(crate) content: String
}


#[derive(Deserialize)]
pub struct SendMessageResponse
{
    pub(crate) message_id: i64
}

#[derive(Serialize)]
pub struct GetMessagesRequest
{
    pub(crate) chat_id: i64,
    pub(crate) message_id: Option<i64>
}

#[derive(Deserialize)]
pub struct GetMessagesResponse
{
    pub(crate) messages: Vec<MessageResponse>
}

#[derive(Serialize)]
pub struct RemoveMessageRequest {
    pub(crate) message_id: i64,
}