use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Channel
{
    pub id: i64,
    pub name: String,
    pub last_edited: i64,
    pub key: Option<[u8; 32]>,
}


#[derive(Deserialize, Debug)]
pub struct ChannelResponse
{
    pub id: i64,
    pub name: String,
    pub last_edited: i64,
    pub key: String,
    pub signature: String,
}

#[derive(Deserialize, Debug)]
pub struct ChannelInfo
{
    pub id: i64,
    pub last_edited: i64,
}


#[derive(Serialize)]
pub struct CreateChannelRequest
{
    pub(crate) name: String
}

#[derive(Deserialize)]
pub struct CreateChannelResponse
{
    pub(crate) channel: ChannelInfo
}

#[derive(Serialize)]
pub struct JoinChannelRequest
{
    pub(crate) id: i64,
    pub(crate) key: String,
    pub(crate) signature: String
}

#[derive(Serialize)]
pub struct RemoveChannelRequest
{
    pub(crate) chat_id: i64,
}

#[derive(Serialize)]
pub struct LeaveChannelRequest
{
    pub(crate) chat_id: i64,
}

#[derive(Serialize)]
pub struct GetChannelListRequest;


#[derive(Deserialize)]
pub struct GetChannelListResponse
{
    pub(crate) channels: Vec<ChannelResponse>
}