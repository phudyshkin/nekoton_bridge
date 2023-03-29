#![allow(unused_variables, dead_code)]
use crate::async_run;
use crate::nekoton_wrapper::core::keystore::models::{SignatureParts, SignedData, SignedDataRaw};
use crate::nekoton_wrapper::crypto::encrypted_key::models::{
    EncryptedKeyCreateInputHelper, EncryptedKeyExportOutputHelper,
};
use crate::nekoton_wrapper::crypto::models::KeySigner;
use crate::nekoton_wrapper::{parse_public_key, HandleError};
use anyhow::Context;
use async_trait::async_trait;
use flutter_rust_bridge::RustOpaque;
use nekoton::core::keystore::{KeyStore, KeyStoreBuilder};
use nekoton::crypto::{
    DerivedKeyCreateInput, DerivedKeyExportParams, DerivedKeyGetPublicKeys, DerivedKeySignParams,
    DerivedKeySigner, DerivedKeyUpdateParams, EncryptedData, EncryptedKeyCreateInput,
    EncryptedKeyGetPublicKeys, EncryptedKeyPassword, EncryptedKeySigner, EncryptedKeyUpdateParams,
    EncryptionAlgorithm, LedgerKeyCreateInput, LedgerKeyGetPublicKeys, LedgerKeySigner,
    LedgerSignInput, LedgerUpdateKeyInput, Signature,
};
use nekoton::external::Storage;
use sha2::Digest;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

pub mod keystore_api;
pub mod models;

#[async_trait]
pub trait KeyStoreApiBoxTrait: Send + Sync + UnwindSafe + RefUnwindSafe {
    /// Get list of json-encoded KeyStoreEntry or throw error
    async fn get_entries(&self) -> Result<String, anyhow::Error>;

    /// Insert new key in keystore. Returns json-encoded KeystoreEntry or throw error.
    /// input - json-encoded action specified for signer eg EncryptedKeyCreateInput or
    ///   DerivedKeyCreateInput or LedgerKeyCreateInput
    async fn add_key(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error>;

    /// Method same as add_key but allows add multiple keys at time.
    /// Returns json-encoded list of KeyStoreEntry or throw error.
    /// input - json-encoded list of inputs, same as in add_key method
    async fn add_keys(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error>;

    /// Update key data.
    /// Returns updated json-encoded KeyStoreEntry r throw error.
    /// input - json-encoded action specified for signer eg EncryptedKeyUpdateParams or
    ///   DerivedKeyUpdateParams or LedgerUpdateKeyInput
    async fn update_key(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error>;

    /// Export key and get its seed phrase and mnemonic type.
    /// THIS METHOD DO NOT WORK for LEDGER.
    /// Returns json-encoded EncryptedKeyExportOutput or DerivedKeyExportOutput or throw error
    async fn export_key(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error>;

    /// Return list of public keys specified for signer or throw error.
    /// input - json-encoded action specified for signer eg EncryptedKeyGetPublicKeys or
    ///   DerivedKeyGetPublicKeys or LedgerKeyGetPublicKeys
    async fn get_public_keys(
        &self,
        signer: KeySigner,
        input: String,
    ) -> Result<Vec<String>, anyhow::Error>;

    /// Encrypt data with specified algorithm and input specified for signer eg EncryptedKeyPassword
    ///   or DerivedKeySignParams or LedgerSignInput.
    /// data - base64 encoded data that must be encrypted.
    /// algorithm - name of algorithm that should be used for encryption, for example ChaCha20Poly1305
    /// public_keys - list of keys that is used for encryption.
    ///
    /// Returns json-encoded list of EncryptedData or throw error.
    async fn encrypt(
        &self,
        signer: KeySigner,
        data: String,
        public_keys: Vec<String>,
        algorithm: String,
        input: String,
    ) -> Result<String, anyhow::Error>;

    /// Decrypt json-encoded EncryptedData in data.
    /// input - json-encoded action for signer eg EncryptedKeyPassword or DerivedKeySignParams or
    ///   LedgerSignInput.
    /// Returns base64-encoded data or throw error.
    async fn decrypt(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
    ) -> Result<String, anyhow::Error>;

    /// Sign data and return base64-encoded signature or throw error.
    /// input - json-encoded action for signer eg EncryptedKeyPassword or DerivedKeySignParams or
    ///   LedgerSignInput.
    /// signature_id - id of transport
    /// data - base64-encoded data that should be signed.
    async fn sign(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
        signature_id: Option<i32>,
    ) -> Result<String, anyhow::Error>;

    /// Same method as sign but data is base64-encoded string that is used as hash via Sha256 algo.
    /// Return SignedData or throw error.
    async fn sign_data(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
        signature_id: Option<i32>,
    ) -> Result<SignedData, anyhow::Error>;

    /// Same method as sign.
    /// data - base64-encoded string.
    /// Return SignedDataRaw or throw error.
    async fn sign_data_raw(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
        signature_id: Option<i32>,
    ) -> Result<SignedDataRaw, anyhow::Error>;

    /// Remove public key from KeyStore and return json-encoded KeyStoreEntry if it was removed.
    async fn remove_key(&self, public_key: String) -> Result<Option<String>, anyhow::Error>;

    /// Remove list of public key from KeyStore and return json-encoded list of KeyStoreEntry's
    /// that were removed.
    async fn remove_keys(&self, public_keys: Vec<String>) -> Result<String, anyhow::Error>;

    /// Check if password cached for specified public_key.
    /// duration - timestamp in milliseconds of expiring key.
    /// Returns true/false or throw error.
    async fn is_password_cached(
        &self,
        public_key: String,
        duration: u64,
    ) -> Result<bool, anyhow::Error>;

    /// Clear KeyStore and remove all entries and all sensitive data.
    async fn clear_keystore(&self) -> Result<String, anyhow::Error>;

    /// Try to reload all stored data.
    async fn reload_keystore(&self) -> Result<String, anyhow::Error>;
}

pub struct KeyStoreApiBox {
    inner_keystore: Arc<KeyStore>,
}

impl UnwindSafe for KeyStoreApiBox {}
impl RefUnwindSafe for KeyStoreApiBox {}

impl KeyStoreApiBox {
    /// Create KeyStoreApiBox or throw error
    pub fn create(
        keystore_builder: KeyStoreBuilder,
        storage: Arc<dyn Storage>,
    ) -> Result<RustOpaque<Arc<dyn KeyStoreApiBoxTrait>>, anyhow::Error> {
        let keystore = async_run!(keystore_builder.load(storage).await).handle_error()?;
        Ok(RustOpaque::new(Arc::new(Self {
            inner_keystore: Arc::new(keystore),
        })))
    }
}

#[async_trait]
impl KeyStoreApiBoxTrait for KeyStoreApiBox {
    /// Get list of json-encoded KeyStoreEntry or throw error
    async fn get_entries(&self) -> Result<String, anyhow::Error> {
        let entries = self.inner_keystore.get_entries().await;

        serde_json::to_string(&entries).handle_error()
    }

    /// Insert new key in keystore. Returns json-encoded KeystoreEntry or throw error.
    /// input - json-encoded action specified for signer eg EncryptedKeyCreateInput or
    ///   DerivedKeyCreateInput or LedgerKeyCreateInput
    async fn add_key(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error> {
        let entry = match signer {
            KeySigner::Encrypted => {
                let input = serde_json::from_str::<EncryptedKeyCreateInputHelper>(&input)
                    .map(
                        |EncryptedKeyCreateInputHelper(encrypted_key_create_input)| {
                            encrypted_key_create_input
                        },
                    )
                    .handle_error()?;

                self.inner_keystore
                    .add_key::<EncryptedKeySigner>(input)
                    .await
                    .handle_error()?
            }
            KeySigner::Derived => {
                let input = serde_json::from_str::<DerivedKeyCreateInput>(&input).handle_error()?;

                self.inner_keystore
                    .add_key::<DerivedKeySigner>(input)
                    .await
                    .handle_error()?
            }
            KeySigner::Ledger => {
                let input = serde_json::from_str::<LedgerKeyCreateInput>(&input).handle_error()?;

                self.inner_keystore
                    .add_key::<LedgerKeySigner>(input)
                    .await
                    .handle_error()?
            }
            _ => {
                panic!("KeySigner:Fake is forbidden")
            }
        };

        serde_json::to_string(&entry).handle_error()
    }

    /// Method same as add_key but allows add multiple keys at time.
    /// Returns json-encoded list of KeyStoreEntry or throw error.
    /// input - json-encoded list of inputs, same as in add_key method
    async fn add_keys(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error> {
        let entries = match signer {
            KeySigner::Encrypted => {
                let input = serde_json::from_str::<Vec<EncryptedKeyCreateInputHelper>>(&input)
                    .handle_error()?
                    .into_iter()
                    .map(
                        |EncryptedKeyCreateInputHelper(encrypted_key_create_input)| {
                            encrypted_key_create_input
                        },
                    )
                    .collect::<Vec<_>>();

                self.inner_keystore
                    .add_keys::<EncryptedKeySigner, Vec<EncryptedKeyCreateInput>>(input)
                    .await
                    .handle_error()?
            }
            KeySigner::Derived => {
                let input =
                    serde_json::from_str::<Vec<DerivedKeyCreateInput>>(&input).handle_error()?;

                self.inner_keystore
                    .add_keys::<DerivedKeySigner, Vec<DerivedKeyCreateInput>>(input)
                    .await
                    .handle_error()?
            }
            KeySigner::Ledger => {
                let input =
                    serde_json::from_str::<Vec<LedgerKeyCreateInput>>(&input).handle_error()?;

                self.inner_keystore
                    .add_keys::<LedgerKeySigner, Vec<LedgerKeyCreateInput>>(input)
                    .await
                    .handle_error()?
            }
            _ => {
                panic!("KeySigner:Fake is forbidden")
            }
        };

        serde_json::to_string(&entries).handle_error()
    }

    /// Update key data.
    /// Returns updated json-encoded KeyStoreEntry r throw error.
    /// input - json-encoded action specified for signer eg EncryptedKeyUpdateParams or
    ///   DerivedKeyUpdateParams or LedgerUpdateKeyInput
    async fn update_key(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error> {
        let entry = match signer {
            KeySigner::Encrypted => {
                let input =
                    serde_json::from_str::<EncryptedKeyUpdateParams>(&input).handle_error()?;

                self.inner_keystore
                    .update_key::<EncryptedKeySigner>(input)
                    .await
                    .handle_error()?
            }
            KeySigner::Derived => {
                let input =
                    serde_json::from_str::<DerivedKeyUpdateParams>(&input).handle_error()?;

                self.inner_keystore
                    .update_key::<DerivedKeySigner>(input)
                    .await
                    .handle_error()?
            }
            KeySigner::Ledger => {
                let input = serde_json::from_str::<LedgerUpdateKeyInput>(&input).handle_error()?;

                self.inner_keystore
                    .update_key::<LedgerKeySigner>(input)
                    .await
                    .handle_error()?
            }
            _ => {
                panic!("KeySigner:Fake is forbidden")
            }
        };

        serde_json::to_string(&entry).handle_error()
    }

    /// Export key and get its seed phrase and mnemonic type.
    /// THIS METHOD DO NOT WORK for LEDGER.
    /// Returns json-encoded EncryptedKeyExportOutput or DerivedKeyExportOutput or throw error
    async fn export_key(&self, signer: KeySigner, input: String) -> Result<String, anyhow::Error> {
        match signer {
            KeySigner::Encrypted => {
                let input = serde_json::from_str::<EncryptedKeyPassword>(&input).handle_error()?;

                let output = self
                    .inner_keystore
                    .export_key::<EncryptedKeySigner>(input)
                    .await
                    .handle_error()?;

                serde_json::to_string(&EncryptedKeyExportOutputHelper(output)).handle_error()
            }
            KeySigner::Derived => {
                let input =
                    serde_json::from_str::<DerivedKeyExportParams>(&input).handle_error()?;

                let output = self
                    .inner_keystore
                    .export_key::<DerivedKeySigner>(input)
                    .await
                    .handle_error()?;

                serde_json::to_string(&output).handle_error()
            }
            KeySigner::Ledger => Err(anyhow::Error::msg(
                "export_key is not allowed for KeySigner::Ledger",
            )),
            _ => {
                panic!("KeySigner:Fake is forbidden")
            }
        }
    }

    /// Return list of public keys specified for signer or throw error.
    /// input - json-encoded action specified for signer eg EncryptedKeyGetPublicKeys or
    ///   DerivedKeyGetPublicKeys or LedgerKeyGetPublicKeys
    async fn get_public_keys(
        &self,
        signer: KeySigner,
        input: String,
    ) -> Result<Vec<String>, anyhow::Error> {
        match signer {
            KeySigner::Encrypted => {
                let input =
                    serde_json::from_str::<EncryptedKeyGetPublicKeys>(&input).handle_error()?;

                Ok(self
                    .inner_keystore
                    .get_public_keys::<EncryptedKeySigner>(input)
                    .await
                    .handle_error()?
                    .into_iter()
                    .map(|e| hex::encode(e.as_bytes()))
                    .collect::<Vec<_>>())
            }
            KeySigner::Derived => {
                let input =
                    serde_json::from_str::<DerivedKeyGetPublicKeys>(&input).handle_error()?;

                Ok(self
                    .inner_keystore
                    .get_public_keys::<DerivedKeySigner>(input)
                    .await
                    .handle_error()?
                    .into_iter()
                    .map(|e| hex::encode(e.as_bytes()))
                    .collect::<Vec<_>>())
            }
            KeySigner::Ledger => {
                let input =
                    serde_json::from_str::<LedgerKeyGetPublicKeys>(&input).handle_error()?;

                Ok(self
                    .inner_keystore
                    .get_public_keys::<LedgerKeySigner>(input)
                    .await
                    .handle_error()?
                    .into_iter()
                    .map(|e| hex::encode(e.as_bytes()))
                    .collect::<Vec<_>>())
            }
            _ => {
                panic!("KeySigner:Fake is forbidden")
            }
        }
    }

    /// Encrypt data with specified algorithm and input specified for signer eg EncryptedKeyPassword
    ///   or DerivedKeySignParams or LedgerSignInput.
    /// data - base64 encoded data that must be encrypted.
    /// algorithm - name of algorithm that should be used for encryption, for example ChaCha20Poly1305
    /// public_keys - list of keys that is used for encryption.
    ///
    /// Returns json-encoded list of EncryptedData or throw error.
    async fn encrypt(
        &self,
        signer: KeySigner,
        data: String,
        public_keys: Vec<String>,
        algorithm: String,
        input: String,
    ) -> Result<String, anyhow::Error> {
        let data = base64::decode(data).handle_error()?;

        let public_keys = public_keys
            .into_iter()
            .map(parse_public_key)
            .collect::<Result<Vec<_>, anyhow::Error>>()
            .context("Bad keys")
            .handle_error()?;

        let algorithm = EncryptionAlgorithm::from_str(&algorithm)
            .context("Bad algorythm")
            .handle_error()?;

        let data = match signer {
            KeySigner::Encrypted => {
                let input = serde_json::from_str::<EncryptedKeyPassword>(&input)
                    .context("Invalid EncryptedKeyPassword")
                    .handle_error()?;

                self.inner_keystore
                    .encrypt::<EncryptedKeySigner>(&data, &public_keys, algorithm, input)
                    .await
                    .context("Failed to encrypt")
                    .handle_error()?
            }
            KeySigner::Derived => {
                let input = serde_json::from_str::<DerivedKeySignParams>(&input)
                    .context("Invalid DerivedKeySignParams")
                    .handle_error()?;

                self.inner_keystore
                    .encrypt::<DerivedKeySigner>(&data, &public_keys, algorithm, input)
                    .await
                    .context("DerivedKeySigner encrypt fail")
                    .handle_error()?
            }
            KeySigner::Ledger => {
                let input = serde_json::from_str::<LedgerSignInput>(&input).handle_error()?;

                self.inner_keystore
                    .encrypt::<LedgerKeySigner>(&data, &public_keys, algorithm, input)
                    .await
                    .handle_error()?
            }
            _ => {
                panic!("KeySigner:Fake is forbidden")
            }
        };

        serde_json::to_string(&data).handle_error()
    }

    /// Decrypt json-encoded EncryptedData in data.
    /// input - json-encoded action for signer eg EncryptedKeyPassword or DerivedKeySignParams or
    ///   LedgerSignInput.
    /// Returns base64-encoded data or throw error.
    async fn decrypt(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
    ) -> Result<String, anyhow::Error> {
        let data = serde_json::from_str::<EncryptedData>(&data).handle_error()?;

        let data = match signer {
            KeySigner::Encrypted => {
                let input = serde_json::from_str::<EncryptedKeyPassword>(&input).handle_error()?;

                self.inner_keystore
                    .decrypt::<EncryptedKeySigner>(&data, input)
                    .await
                    .handle_error()?
            }
            KeySigner::Derived => {
                let input = serde_json::from_str::<DerivedKeySignParams>(&input).handle_error()?;

                self.inner_keystore
                    .decrypt::<DerivedKeySigner>(&data, input)
                    .await
                    .handle_error()?
            }
            KeySigner::Ledger => {
                let input = serde_json::from_str::<LedgerSignInput>(&input).handle_error()?;

                self.inner_keystore
                    .decrypt::<LedgerKeySigner>(&data, input)
                    .await
                    .handle_error()?
            }
            _ => {
                panic!("KeySigner:Fake is forbidden")
            }
        };

        let data = base64::encode(data);

        serde_json::to_string(&data).handle_error()
    }

    /// Sign data and return base64-encoded signature or throw error.
    /// input - json-encoded action for signer eg EncryptedKeyPassword or DerivedKeySignParams or
    ///   LedgerSignInput.
    /// signature_id - id of transport
    /// data - base64-encoded data that should be signed.
    async fn sign(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
        signature_id: Option<i32>,
    ) -> Result<String, anyhow::Error> {
        let data = base64::decode(&data).handle_error()?;
        let signature = sign(
            self.inner_keystore.clone(),
            signer,
            &data,
            input,
            signature_id,
        )
        .await?;

        Ok(base64::encode(signature))
    }

    /// Same method as sign but data is base64-encoded string that is used as hash via Sha256 algo.
    /// Return SignedData or throw error.
    async fn sign_data(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
        signature_id: Option<i32>,
    ) -> Result<SignedData, anyhow::Error> {
        let data = base64::decode(data).handle_error()?;
        let hash: [u8; 32] = sha2::Sha256::digest(&data).into();

        let signature = sign(
            self.inner_keystore.clone(),
            signer,
            &hash,
            input,
            signature_id,
        )
        .await?;

        Ok(SignedData {
            data_hash: hex::encode(hash),
            signature: base64::encode(signature),
            signature_hex: hex::encode(signature),
            signature_parts: SignatureParts {
                high: format!("0x{}", hex::encode(&signature[..32])),
                low: format!("0x{}", hex::encode(&signature[32..])),
            },
        })
    }

    /// Same method as sign.
    /// data - base64-encoded string.
    /// Return SignedDataRaw or throw error.
    async fn sign_data_raw(
        &self,
        signer: KeySigner,
        data: String,
        input: String,
        signature_id: Option<i32>,
    ) -> Result<SignedDataRaw, anyhow::Error> {
        let data = base64::decode(data).handle_error()?;

        let signature = sign(
            self.inner_keystore.clone(),
            signer,
            &data,
            input,
            signature_id,
        )
        .await?;

        Ok(SignedDataRaw {
            signature: base64::encode(signature),
            signature_hex: hex::encode(signature),
            signature_parts: SignatureParts {
                high: format!("0x{}", hex::encode(&signature[..32])),
                low: format!("0x{}", hex::encode(&signature[32..])),
            },
        })
    }

    /// Remove public key from KeyStore and return json-encoded KeyStoreEntry if it was removed.
    async fn remove_key(&self, public_key: String) -> Result<Option<String>, anyhow::Error> {
        let public_key = parse_public_key(public_key)?;

        let entry = self
            .inner_keystore
            .remove_key(&public_key)
            .await
            .handle_error()?;

        Ok(match entry {
            None => None,
            Some(e) => Some(serde_json::to_string(&e).handle_error()?),
        })
    }

    /// Remove list of public key from KeyStore and return json-encoded list of KeyStoreEntry's
    /// that were removed.
    async fn remove_keys(&self, public_keys: Vec<String>) -> Result<String, anyhow::Error> {
        let public_keys = public_keys
            .into_iter()
            .map(parse_public_key)
            .collect::<Result<Vec<_>, anyhow::Error>>()
            .handle_error()?;

        let entries = self
            .inner_keystore
            .remove_keys(&public_keys)
            .await
            .handle_error()?;

        serde_json::to_string(&entries).handle_error()
    }

    /// Check if password cached for specified public_key.
    /// duration - timestamp in milliseconds of expiring key.
    /// Returns true/false or throw error.
    async fn is_password_cached(
        &self,
        public_key: String,
        duration: u64,
    ) -> Result<bool, anyhow::Error> {
        let id = parse_public_key(public_key).handle_error()?.to_bytes();
        let duration = Duration::from_millis(duration);
        let is_cached = self.inner_keystore.is_password_cached(&id, duration);

        Ok(is_cached)
    }

    /// Clear KeyStore and remove all entries and all sensitive data.
    async fn clear_keystore(&self) -> Result<String, anyhow::Error> {
        let _ = self.inner_keystore.clear().await.handle_error()?;
        Ok(serde_json::Value::Null.to_string())
    }

    /// Try to reload all stored data.
    async fn reload_keystore(&self) -> Result<String, anyhow::Error> {
        let _ = self.inner_keystore.reload().await.handle_error()?;
        Ok(serde_json::Value::Null.to_string())
    }
}
/// Verify if data is valid with specified signers and connection or not.
/// Return true/false or throw error.
pub fn verify_data(keystore_builder: KeyStoreBuilder, data: String) -> bool {
    keystore_builder.verify(&data).is_ok()
}

async fn sign(
    keystore: Arc<KeyStore>,
    signer: KeySigner,
    data: &[u8],
    input: String,
    signature_id: Option<i32>,
) -> Result<Signature, anyhow::Error> {
    let signature_id = signature_id;

    match signer {
        KeySigner::Encrypted => {
            let input = serde_json::from_str::<EncryptedKeyPassword>(&input).handle_error()?;

            keystore
                .sign::<EncryptedKeySigner>(data, signature_id, input)
                .await
                .handle_error()
        }
        KeySigner::Derived => {
            let input = serde_json::from_str::<DerivedKeySignParams>(&input).handle_error()?;

            keystore
                .sign::<DerivedKeySigner>(data, signature_id, input)
                .await
                .handle_error()
        }
        KeySigner::Ledger => {
            let input = serde_json::from_str::<LedgerSignInput>(&input).handle_error()?;

            keystore
                .sign::<LedgerKeySigner>(data, signature_id, input)
                .await
                .handle_error()
        }
        _ => {
            panic!("KeySigner:Fake is forbidden")
        }
    }
}
