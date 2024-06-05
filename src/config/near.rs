use near_fetch::signer::KeyRotatingSigner;
use near_crypto::InMemorySigner;
use serde::Deserialize;
use std::{fmt::Debug, path::PathBuf, sync::Arc};
use ::config::{Config, File as ConfigFile};

// region: --- ApiKey
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct ApiKey(pub near_jsonrpc_client::auth::ApiKey);

impl From<ApiKey> for near_jsonrpc_client::auth::ApiKey {
    fn from(api_key: ApiKey) -> Self {
        api_key.0
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::str::FromStr for ApiKey {
    type Err = color_eyre::Report;

    fn from_str(api_key: &str) -> Result<Self, Self::Err> {
        Ok(Self(near_jsonrpc_client::auth::ApiKey::new(api_key)?))
    }
}

impl serde::ser::Serialize for ApiKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.0.to_str().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for ApiKey {
    fn deserialize<D>(deserializer: D) -> Result<ApiKey, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|err: color_eyre::Report| serde::de::Error::custom(err.to_string()))
    }
}

// endregion: --- ApiKey

#[derive(Debug, Clone, Deserialize)]
pub struct NearNetworkConfig {
    rpc_url: url::Url,
    rpc_api_key: Option<ApiKey>,
    pub whitelisted_contracts: Vec<String>,
    pub whitelisted_senders: Vec<String>,
    keys_filename: PathBuf,
}

impl NearNetworkConfig {
    pub fn init() -> Self {
        Config::builder()
            .add_source(ConfigFile::with_name("config.toml"))
            .build()
            .unwrap()
            .try_deserialize::<NearNetworkConfig>()
            .unwrap()
    }

    pub fn rpc_client(&self) -> near_fetch::Client {
        near_fetch::Client::from_client(self.raw_rpc_client())
    }

    // TODO: more specific setting ref. near_core github
    pub fn raw_rpc_client(&self) -> near_jsonrpc_client::JsonRpcClient {
        let mut json_rpc_client =
            near_jsonrpc_client::JsonRpcClient::connect(self.rpc_url.as_ref());
        if let Some(rpc_api_key) = &self.rpc_api_key {
            json_rpc_client = json_rpc_client.header(rpc_api_key.0.clone());
        };
        json_rpc_client
    }

    pub fn init_rotating_signer(&self) -> KeyRotatingSignerWrapper {
        let keys_file = std::fs::File::open(&self.keys_filename)
            .expect("Failed to open keys file");
        let signers: Vec<InMemorySigner> = serde_json::from_reader(keys_file)
            .expect("Failed to parse keys file");

        KeyRotatingSignerWrapper::from_signers(signers)
    }
}


// region: --- KeyRotatingSignerWrapper

#[derive(Clone)]
pub struct KeyRotatingSignerWrapper{
    signer: Arc<KeyRotatingSigner>,
}

impl Debug for KeyRotatingSignerWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyRotatingSignerWrapper(...)")
    }
}

impl KeyRotatingSignerWrapper {
    pub fn from_signers(signers: Vec<InMemorySigner>) -> Self {
        KeyRotatingSignerWrapper {
            signer: Arc::new(KeyRotatingSigner::from_signers(signers)),
        }
    }

    pub fn inner(&self) -> Arc<KeyRotatingSigner> {
        Arc::clone(&self.signer)
    }
}
// endregion: --- KeyRotatingSignerWrapper