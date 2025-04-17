use base64::Engine;
use base64::engine::general_purpose;
use rand::{rng, Rng};
use rand::distr::Alphanumeric;
use crate::utils::rsa_api;
use crate::WhisperClient;

const SERVER: &str = "http://localhost:8080";
const PRIVATE_KEY: &str = "LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQpNSUlFcEFJQkFBS0NBUUVBc0VTQys2VDJSMS9kSWJPYzQzZkNhQUpiTUhGVjUxRmFyTldZdGhNajhueWRjUkxECnU0QlhsZlFzTmkxR3RmcU1EU05BTlNEWEdsZlpPWEdraEQzeHRYRGlLbTVqSnc0UFo0RUJFbzMvaGZHVDc2eUMKR0JNSG5ZdUMreHI1c3ZiRWc3bmp3MGpadjdCc3BXbHNhek55Zkp5d0xDejFaZ3R0d3lIenVmN1NGY3JFL3VSNQp4ejhvQ05HSllKc1pWdlIxOGlOZ2hkMy9UcW9wUkRzeklyc1ViWHMycHpjbU1DekYwZGVTVnp0bFBpNms5YUhLClZUMnRoYm4wV0l5OVJpUjZxRVl4ZTE4V2Z5N2V6b21zQWE4eVVEUUVpZmNHbDBHa0hNRit3MEJHMmM1VnZUdlEKRmIwUkltNmUxYU9BYjNYdklUQnh4QWNPZ3FyRDliSVBGYUJNV1FJREFRQUJBb0lCQUQzdGhCdEhKS09FNFpzQwprSjgySng0Rm5lWVNHMzB1anUza0NVZ0d4MzFkSEs5cVNVd3AxaHUvOG55Q1FiS1R1UHF3cE5GRm5XMEs2aTJOCmFLQnhadWM1SnF5RHBlQkZ4MUcwK0J4OXVRVmxEM1dJSlJpeUx0M0I4eDJucDR5aHQvOFRveHFzRUp5UkxrcWIKRkZWQmVQcWd6aUJuRnp4bnpZTmQvVlFlVzg3OW5oQ3d5VzJYeFFsTmtZeXRMZTFoMGNaMkwrczNpZTNpcHBMcAo4RHNVRnhMcnA3YXZGbFQxZEN0STNUNlhEL3d3TWdYSkJsOHZzNWdNblM3ZkZBWFBoOGxaM2V4OXkwUjRWUGhSCnpiVGN0SWxFODF1di84ZWIySnU1azR6T3JJWUVxanlud05hTElmSWF0TW5kWnJwM1d5dndaZGcvdnJKU1VoWUcKL0xObTZ4RUNnWUVBMmtaSmc1OS9pRVBTWVN2ZWVTZnRzL2lSOGtidWIreVpyRWF5OXVrU1kzbnp0Y3dRSHVxeApiMUJGaDZYOXZhQ203S2xDWDZUVkpJd1F4Q3BsSDZXU0pVOE93MjBRaGN2WUhPbHhuaCt5d1RNZVR1ZXRjMVJqCmJIaEFPVGhhanlPM2FpNTRhS2xwMnFYekNwNTNBR3ZzL0lLOXBiRTNpcXFYWjJKMlJmVHErM1VDZ1lFQXpydVoKQ1doMGd6MHd3ZlY4ejVCWkh1WWZYUDh0bG5YeTNTV05jcm9wU010aVMxYjFIakpxakR1ZjQ2eEtMY0ZsWEtPTgpHYTJ1KzczYVhMRlpMcmtSLzY3VTZEdEFHbHFZd0VPdUp0TXUyUmU3UVRlOGZVTXBTVG00NTZINS9tcVNWTHZUCmVjUGZDdm8vQ200NjB6UG84cFVSdjF1bXh5bzN3akZuQVhYS1JOVUNnWUFLWnIwTUU2YXRKS1k2MFM5WjBLaEkKSWprNk5WMFpZa24wWnE0U2pBcS9TTWx2U1ZrZlVBbkNoeTI1Q0JUdVcyQjQrSnZjR091N1FSMXZhNkhELzB6VwprRXpnelNxelpZSlg1bHZ1c0E0Qm5PRDkyNVp0WDRFWll5V1VWSFlrU2d4c2QraHUvRnU1K3B4NVRoSFhxRXp6CnYzc3dFU0RYYjhlbE9wRHVSbnlJSFFLQmdRQ1hHcXVySXJ3MnlOMEFpQXhvTWx2UnArWTR0Uk4vTEVzTnRVc0UKRm1uaW1UWUpWMC9tZUhkRWRMaFRVelVNNkpUTDk0ZEV3NXhveU1YNGhuQm5KRUt4bmZwa25Cb29xUnVKUEc3bwpWZWVpS2lSbWNQVEdvZlpsWFZsM2hQOFRKSlk4ZE9VSDFWRUwxd21JK0RUcTlzQkh2d212MHErK1YyOVY4NElVCm9TSHMyUUtCZ1FDdTN3UFNTTWpjZ2FvbTlUaFBLd0h4ZXVmdlpmTkszcDYyd0tpblVHeVIrSlVod1RzUWxlT0sKaWl1MGxrcDJPSHhJNXlhdTFoL1BCOTgzUFhvZXZVZVNDRXJDcFhPWkNWNXJLOGxvcG9JZmRuNjgveWNkOUc3dAoxY3hrcmRBZ2pNZDZhWk5BWEJaeG9EamFMNGZTZlNTcVRGVzE5ZWJyRkhlWURoeGtXdytOV0E9PQotLS0tLUVORCBSU0EgUFJJVkFURSBLRVktLS0tLQo=";


fn get_random_string() -> String
{
    rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>()
}

async fn get_random_user() -> WhisperClient
{
    WhisperClient::register(
        SERVER.to_string(),
        get_random_string(),
        PRIVATE_KEY.to_string()
    ).await.unwrap()
}

#[tokio::test]
async fn register()  {
    let result = WhisperClient::register(
        SERVER.to_string(),
        get_random_string(),
        PRIVATE_KEY.to_string()
    ).await;

    if let Err(ref e) = result {
        eprintln!("{e}")
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn login() {
    let username = get_random_string();

    WhisperClient::register(
        SERVER.to_string(),
        username.clone(),
        PRIVATE_KEY.to_string()
    ).await.unwrap();

    let result =  WhisperClient::login(
        SERVER.to_string(),
        username.clone(),
        PRIVATE_KEY.to_string()
    ).await;

    if let Err(ref e) = result {
        eprintln!("{e}")
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_channel() {
    let client = get_random_user().await;

    let result = client.create_channel("test".to_string()).await;

    if let Ok(ref channel) = result
    {
        println!("Chat id: {}", channel.id);
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn join_channel() {
    let client = get_random_user().await;

    let channel = client.create_channel("test".to_string()).await.unwrap();
    client.leave_channel(channel.id).await.unwrap();

    let result = client.join_channel(channel.id, "1".to_string(), "1".to_string()).await;

    if let Ok(_) = result
    {
        println!("Chat id: {}", channel.id);
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}


#[tokio::test]
async fn leave_channel() {
    let client = get_random_user().await;

    let channel = client.create_channel("test".to_string()).await.unwrap();
    println!("Successfully joined channel with id: {}", channel.id);

    let result = client.leave_channel(channel.id).await;
    if let Ok(_) = result
    {
        println!("Successfully left channel with id: {}", channel.id);
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn remove_channel() {
    let client = get_random_user().await;

    let channel = client.create_channel("test".to_string()).await.unwrap();
    println!("Successfully joined channel with id: {}", channel.id);

    let result = client.remove_channel(channel.id).await;
    if let Ok(_) = result
    {
        println!("Successfully removed channel with id: {}", channel.id);
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channels() {
    let client = get_random_user().await;

    client.create_channel("test".to_string()).await.unwrap();

    let result = client.get_channels().await;

    if let Ok(ref data) = result
    {
        for channel in data.iter()
        {
            println!("Channel: {:#?}", channel);
        }
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_message() {
    let client = get_random_user().await;

    let channel = client.create_channel("test".to_string()).await.unwrap();

    let result = client.send_message(channel.id, "test".to_string(), channel.key.unwrap()).await;

    if let Ok(ref data) = result
    {
        println!("Message sent successfully with id: {data}")
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn remove_message() {
    let client = get_random_user().await;

    let channel = client.create_channel("test".to_string()).await.unwrap();
    let message_id = client.send_message(channel.id, "test".to_string(), channel.key.unwrap()).await.unwrap();

    let result = client.remove_message(message_id).await;

    if let Ok(_) = result
    {
        println!("Message removed successfully with id: {message_id}")
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_messages() {
    let client = get_random_user().await;

    let channel = client.create_channel("test".to_string()).await.unwrap();

    client.send_message(channel.id, "test".to_string(), channel.key.unwrap()).await.unwrap();

    let result = client.get_messages(channel.id, None, channel.key.unwrap()).await;

    if let Ok(ref messages) = result
    {
        for message in messages {
            println!("Message: {:#?}", message);
        }
    }

    if let Err(ref e) = result
    {
        eprintln!("{e}");
    }

    assert!(result.is_ok());
}


#[tokio::test]
async fn generate_key()  {
    let random_key = general_purpose::STANDARD.encode(rsa_api::generate_random_key());
    let random_key_two = general_purpose::STANDARD.encode(rsa_api::generate_random_key());

    println!("Key one: {random_key}");
    println!("Key two: {random_key_two}");

    assert_ne!(random_key, random_key_two);
}

#[tokio::test]
async fn generate_signature()  {
    let random_key = rsa_api::generate_random_key();
    let random_key_encrypted = rsa_api::encrypt_bytes(PRIVATE_KEY, Vec::from(random_key)).unwrap();

    let signature = rsa_api::generate_signature(random_key_encrypted, PRIVATE_KEY).unwrap();

    println!("Signature: {signature}");
}

#[tokio::test]
async fn check_signature_valid()  {
    let random_key = rsa_api::generate_random_key();
    let random_key_encrypted = rsa_api::encrypt_bytes(PRIVATE_KEY, Vec::from(random_key)).unwrap();

    let signature = rsa_api::generate_signature(random_key_encrypted.clone(), PRIVATE_KEY).unwrap();

    let key = rsa_api::check_signature(random_key_encrypted, PRIVATE_KEY, &signature);

    if let Ok(key) = key
    {
        let key_encoded = general_purpose::STANDARD.encode(key);
        println!("Key: {key_encoded}");
    }

    if let Err(ref e) = key
    {
        eprintln!("{e}");
    }

    assert!(key.is_ok());
}

#[tokio::test]
async fn check_signature_invalid()  {
    let random_key = rsa_api::generate_random_key();
    let random_key_two = rsa_api::generate_random_key();

    let random_key_encrypted = rsa_api::encrypt_bytes(PRIVATE_KEY, Vec::from(random_key)).unwrap();
    let random_key_two_encrypted = rsa_api::encrypt_bytes(PRIVATE_KEY, Vec::from(random_key_two)).unwrap();

    let signature = rsa_api::generate_signature(random_key_encrypted.clone(), PRIVATE_KEY).unwrap();

    let key = rsa_api::check_signature(random_key_two_encrypted.clone(), PRIVATE_KEY, &signature);

    println!("Key one: {random_key_encrypted}");
    println!("Key two: {random_key_two_encrypted}");

    assert!(!key.is_ok());
}