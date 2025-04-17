use base64::engine::general_purpose;
use base64::Engine;
use cosmian_crypto_core::{Dem, FixedSizeCBytes, Instantiable, Nonce, RandomFixedSizeCBytes, SymmetricKey, XChaCha20Poly1305};
use cosmian_crypto_core::reexport::rand_core::OsRng;

pub struct Crypto {
    crypto: XChaCha20Poly1305,
}

impl Crypto {
    pub fn new(key: [u8; 32]) -> Result<Crypto, String> {
        let secret_key = SymmetricKey::try_from_slice(&key)
            .map_err(|_| "Invalid key size")?;

        let crypto = XChaCha20Poly1305::new(&secret_key);

        Ok(Crypto { crypto })
    }

    pub fn encrypt_string(&self, data: String) -> Result<String, String> {
        let nonce = Nonce::<24>::new(&mut OsRng);

        let ciphertext = self.crypto.encrypt(&nonce, data.as_bytes(), None)
            .map_err(|_| "Cannot encrypt data")?;

        let mut full_payload = nonce.0.to_vec();
        full_payload.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(full_payload))
    }

    pub fn decrypt_string(&self, data: String) -> Result<String, String> {
        let decoded = general_purpose::STANDARD.decode(&data)
            .map_err(|_| "Cannot decode data")?;

        if decoded.len() < 24 {
            return Err("Data too short to contain nonce".into());
        }

        let (nonce_bytes, ciphertext) = decoded.split_at(24);
        let nonce = Nonce::<24>::try_from_slice(nonce_bytes)
            .map_err(|_| "Invalid nonce")?;

        let plaintext = self.crypto.decrypt(&nonce, ciphertext, None)
            .map_err(|_| "Cannot decrypt data")?;

        Ok(String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8")?)
    }
}
