#![allow(unused_variables, dead_code)]

use crate::async_run;
use anyhow::Error;
use flutter_rust_bridge::RustOpaque;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;

use crate::nekoton_wrapper::core::accounts_storage::models::{
    AccountToAddHelper, AssetsListHelper,
};
use crate::nekoton_wrapper::{parse_address, HandleError};
use async_trait::async_trait;
use nekoton::core::accounts_storage::AccountsStorage;
use nekoton::external::Storage;

pub mod accounts_storage_api;
pub mod models;

#[async_trait]
pub trait AccountsStorageBoxTrait: Send + Sync + UnwindSafe + RefUnwindSafe {
    /// Get list of accounts.
    /// Returns json-encoded List of AssetsList or throw error
    async fn get_entries(&self) -> Result<String, anyhow::Error>;

    /// Add new account to storage and return its instance.
    /// account - json-encoded AccountToAdd.
    /// Return json-encoded AssetsList or throw error.
    async fn add_account(&self, account: String) -> Result<String, anyhow::Error>;

    /// Add list of new accounts to storage and return it instances.
    /// account - json-encoded list of AccountToAdd.
    /// Return json-encoded list of AssetsList or throw error.
    async fn add_accounts(&self, accounts: String) -> Result<String, anyhow::Error>;

    /// Rename existed account and return its renamed instance.
    /// account_address - address of account
    /// name - new name of account
    /// Return json-encoded AssetsList or throw error.
    async fn rename_account(
        &self,
        account_address: String,
        name: String,
    ) -> Result<String, anyhow::Error>;

    /// Add token wallet signature to account (add new token to account aka enable it via slider).
    /// account_address - address of account
    /// network_group - name of network group where this token must be visible, could be found in
    ///   connection info
    /// root_token_contract - address of token in blockchain.
    /// Return updated AssetsList or throw error.
    async fn add_token_wallet(
        &self,
        account_address: String,
        network_group: String,
        root_token_contract: String,
    ) -> Result<String, anyhow::Error>;

    /// Remove token wallet signature from account (remove token from account aka disable it via slider).
    /// account_address - address of account
    /// network_group - name of network group where this token must be visible, could be found in
    ///   connection info
    /// root_token_contract - address of token in blockchain.
    /// Return updated AssetsList or throw error.
    async fn remove_token_wallet(
        &self,
        account_address: String,
        network_group: String,
        root_token_contract: String,
    ) -> Result<String, anyhow::Error>;

    /// Remove account from storage and return its instance if it was removed.
    /// account_address - address of account
    /// Return json-encoded AssetsList that was removed or null or throw error.
    async fn remove_account(
        &self,
        account_address: String,
    ) -> Result<Option<String>, anyhow::Error>;

    /// Remove list of account from storage and return it instances if it were removed.
    /// account_addresses - list of addresses of accounts.
    /// Return json-encoded list of AssetsList that were removed or throw error.
    async fn remove_accounts(
        &self,
        account_addresses: Vec<String>,
    ) -> Result<String, anyhow::Error>;

    /// Clear storage and remove all data.
    /// Returns true or throw error
    async fn clear(&self) -> Result<bool, anyhow::Error>;

    /// Reload storage and read all data again.
    /// Returns true or throw error.
    async fn reload(&self) -> Result<bool, anyhow::Error>;
}

pub struct AccountsStorageBox {
    inner_storage: Arc<AccountsStorage>,
}

impl UnwindSafe for AccountsStorageBox {}
impl RefUnwindSafe for AccountsStorageBox {}

impl AccountsStorageBox {
    /// Create AccountsStorageBox or throw error
    pub fn create(
        storage: Arc<dyn Storage>,
    ) -> Result<RustOpaque<Arc<dyn AccountsStorageBoxTrait>>, anyhow::Error> {
        let storage = async_run!(AccountsStorage::load(storage).await).handle_error()?;
        Ok(RustOpaque::new(Arc::new(Self {
            inner_storage: Arc::new(storage),
        })))
    }
}

#[async_trait]
impl AccountsStorageBoxTrait for AccountsStorageBox {
    /// Get list of accounts.
    /// Returns json-encoded List of AssetsList or throw error
    async fn get_entries(&self) -> Result<String, Error> {
        let entries = self
            .inner_storage
            .stored_data()
            .await
            .accounts()
            .values()
            .cloned()
            .map(AssetsListHelper)
            .collect::<Vec<_>>();

        serde_json::to_string(&entries).handle_error()
    }

    /// Add new account to storage and return its instance.
    /// account - json-encoded AccountToAdd.
    /// Return json-encoded AssetsList or throw error.
    async fn add_account(&self, account: String) -> Result<String, anyhow::Error> {
        let new_account = serde_json::from_str::<AccountToAddHelper>(&account)
            .map(|AccountToAddHelper(account_to_add)| account_to_add)
            .handle_error()?;

        let entry = self
            .inner_storage
            .add_account(new_account)
            .await
            .handle_error()?;

        serde_json::to_string(&AssetsListHelper(entry)).handle_error()
    }

    /// Add list of new accounts to storage and return it instances.
    /// account - json-encoded list of AccountToAdd.
    /// Return json-encoded list of AssetsList or throw error.
    async fn add_accounts(&self, accounts: String) -> Result<String, anyhow::Error> {
        let new_accounts = serde_json::from_str::<Vec<AccountToAddHelper>>(&accounts)
            .handle_error()?
            .into_iter()
            .map(|AccountToAddHelper(account_to_add)| account_to_add)
            .collect::<Vec<_>>();

        let entries = self
            .inner_storage
            .add_accounts(new_accounts)
            .await
            .handle_error()?
            .into_iter()
            .map(AssetsListHelper)
            .collect::<Vec<_>>();

        serde_json::to_string(&entries).handle_error()
    }

    /// Rename existed account and return its renamed instance.
    /// account_address - address of account
    /// name - new name of account
    /// Return json-encoded AssetsList or throw error.
    async fn rename_account(
        &self,
        account_address: String,
        name: String,
    ) -> Result<String, anyhow::Error> {
        let entry = self
            .inner_storage
            .rename_account(&account_address, name)
            .await
            .handle_error()?;

        serde_json::to_string(&AssetsListHelper(entry)).handle_error()
    }

    /// Add token wallet signature to account (add new token to account aka enable it via slider).
    /// account_address - address of account
    /// network_group - name of network group where this token must be visible, could be found in
    ///   connection info
    /// root_token_contract - address of token in blockchain.
    /// Return updated AssetsList or throw error.
    async fn add_token_wallet(
        &self,
        account_address: String,
        network_group: String,
        root_token_contract: String,
    ) -> Result<String, anyhow::Error> {
        let root_token_contract = parse_address(root_token_contract)?;

        let entry = self
            .inner_storage
            .add_token_wallet(&account_address, &network_group, root_token_contract)
            .await
            .handle_error()?;

        serde_json::to_string(&AssetsListHelper(entry)).handle_error()
    }

    /// Remove token wallet signature from account (remove token from account aka disable it via slider).
    /// account_address - address of account
    /// network_group - name of network group where this token must be visible, could be found in
    ///   connection info
    /// root_token_contract - address of token in blockchain.
    /// Return updated AssetsList or throw error.
    async fn remove_token_wallet(
        &self,
        account_address: String,
        network_group: String,
        root_token_contract: String,
    ) -> Result<String, anyhow::Error> {
        let root_token_contract = parse_address(root_token_contract)?;

        let entry = self
            .inner_storage
            .remove_token_wallet(&account_address, &network_group, &root_token_contract)
            .await
            .handle_error()?;

        serde_json::to_string(&AssetsListHelper(entry)).handle_error()
    }

    /// Remove account from storage and return its instance if it was removed.
    /// account_address - address of account
    /// Return json-encoded AssetsList that was removed or null or throw error.
    async fn remove_account(
        &self,
        account_address: String,
    ) -> Result<Option<String>, anyhow::Error> {
        let entry = self
            .inner_storage
            .remove_account(&account_address)
            .await
            .handle_error()?
            .map(AssetsListHelper);

        match entry {
            None => Ok(None),
            Some(v) => Ok(Some(serde_json::to_string(&v).handle_error()?)),
        }
    }

    /// Remove list of account from storage and return it instances if it were removed.
    /// account_addresses - list of addresses of accounts.
    /// Return json-encoded list of AssetsList that were removed or throw error.
    async fn remove_accounts(
        &self,
        account_addresses: Vec<String>,
    ) -> Result<String, anyhow::Error> {
        let accounts = account_addresses
            .iter()
            .map(|item| item.as_str())
            .collect::<Vec<&str>>();
        let entries = self
            .inner_storage
            .remove_accounts(accounts)
            .await
            .handle_error()?
            .into_iter()
            .map(AssetsListHelper)
            .collect::<Vec<_>>();

        serde_json::to_string(&entries).handle_error()
    }

    /// Clear storage and remove all data.
    /// Returns true or throw error
    async fn clear(&self) -> Result<bool, anyhow::Error> {
        self.inner_storage.clear().await.handle_error()?;
        Ok(true)
    }

    /// Reload storage and read all data again.
    /// Returns true or throw error.
    async fn reload(&self) -> Result<bool, anyhow::Error> {
        self.inner_storage.reload().await.handle_error()?;
        Ok(true)
    }
}

/// Check if data is correct for storage.
pub fn accounts_storage_verify_data(data: String) -> bool {
    AccountsStorage::verify(&data).is_ok()
}
