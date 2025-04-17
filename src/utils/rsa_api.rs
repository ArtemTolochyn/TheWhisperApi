use base64::Engine;
use base64::engine::general_purpose;
use rand::{thread_rng, Rng};
use rsa::pkcs8::{EncodePublicKey};
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pss::{BlindedSigningKey, Signature};
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use sha2::{Sha256};


pub fn generate_random_key() -> [u8; 32] {
    let mut rng = thread_rng();
    rng.gen::<[u8; 32]>()
}

pub fn check_signature(key: String, private_key: &str, signature: &str) -> Result<[u8; 32], String>
{
    let private_key_pem = get_private_key(private_key)?;
    let signing_key = BlindedSigningKey::<Sha256>::new(private_key_pem);
    let verifying_key = signing_key.verifying_key();

    let signature_bytes = general_purpose::STANDARD.decode(signature)
        .map_err(|e| format!("Cannot decode signature from base64: {}", e))?;


    let signature_object = Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| "Cannot decode signature")?;

    verifying_key.verify(key.as_bytes(), &signature_object)
        .map_err(|_| "Cannot verify signature")?;

    let key_decrypted = decrypt_bytes(private_key, &key)?;

    let key_array: [u8; 32] = key_decrypted.as_slice()
        .try_into()
        .map_err(|_| "Invalid key length")?;

    Ok(key_array)
}

pub fn generate_signature(key: String, private_key: &str) -> Result<String, String>
{
    let private_key_pem = get_private_key(private_key)?;
    let signing_key = BlindedSigningKey::<Sha256>::new(private_key_pem);

    let mut rng = thread_rng();
    let signature = signing_key.sign_with_rng(&mut rng, key.as_bytes());

    Ok(general_purpose::STANDARD.encode(signature.to_bytes()))
}

pub fn get_public_key_base64(private_key: &str) -> Result<String, String>
{
    let public_key = get_public_key(private_key)?;
    let public_key_pem = public_key.to_public_key_pem(Default::default())
        .map_err(|_| "Failed to encode public key to PEM")?;

    Ok(general_purpose::STANDARD.encode(public_key_pem.as_bytes()))
}

pub fn get_public_key(private_key: &str) -> Result<RsaPublicKey, String>
{
    let private_key = get_private_key(private_key)?;

    Ok(private_key.to_public_key())
}

pub fn get_private_key(private_key: &str) -> Result<RsaPrivateKey, String>
{
    let private_key_pem = decode_private_key_pem(private_key)?;
    RsaPrivateKey::from_pkcs1_pem(&private_key_pem)
        .map_err(|_| "Invalid private key".to_string())
}

pub fn encrypt_string(private_key: &str, data: &str) -> Result<String, String>
{
    let mut rng = thread_rng();
    let public_key_pem = get_public_key(private_key)?;
    let padding = Oaep::new::<Sha256>();

    let data_bytes = data.as_bytes();
    let encrypted = public_key_pem.encrypt(&mut rng, padding, data_bytes)
        .map_err(|_| "Invalid public key")?;

    Ok(general_purpose::STANDARD.encode(encrypted))
}

pub fn encrypt_bytes(private_key: &str, data: Vec<u8>) -> Result<String, String>
{
    let mut rng = thread_rng();
    let public_key_pem = get_public_key(private_key)?;
    let padding = Oaep::new::<Sha256>();

    let encrypted = public_key_pem.encrypt(&mut rng, padding, &data)
        .map_err(|_| "Invalid public key")?;

    Ok(general_purpose::STANDARD.encode(encrypted))
}

pub fn decrypt_string(private_key: &str, data: &str) -> Result<String, String>
{
    let private_key_pem = get_private_key(private_key)?;
    let padding = Oaep::new::<Sha256>();

    let data_bytes = general_purpose::STANDARD.decode(data)
        .map_err(|_| "Cannot decode data from base64")?;

    let decrypted = private_key_pem.decrypt(padding, &data_bytes).map_err(|_| "Invalid private key")?;

    String::from_utf8(decrypted)
        .map_err(|_| "Invalid UTF-8".to_string())
}

pub fn decrypt_bytes(private_key: &str, data: &str) -> Result<Vec<u8>, String>
{
    let private_key_pem = get_private_key(private_key)?;
    let padding = Oaep::new::<Sha256>();

    let data_bytes = general_purpose::STANDARD.decode(data)
        .map_err(|_| "Cannot decode data from base64")?;

    private_key_pem.decrypt(padding, &data_bytes)
        .map_err(|_| "Invalid private key".to_string())
}


fn decode_private_key_pem(private_key: &str) -> Result<String, String>
{
    let private_key = general_purpose::STANDARD.decode(private_key)
        .map_err(|_| "Invalid private key")?;

    String::from_utf8(private_key)
        .map_err(|_| "Invalid private key".to_string())
}