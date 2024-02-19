use std::sync::Arc;
use std::{path, str::FromStr};

use bip39::Mnemonic;
use ldk_node::io::sqlite_store::SqliteStore;
use thiserror::Error;

use ldk_node::{Builder, Network, Node};

#[derive(Error, Clone, Debug)]
pub enum SdkError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("LDK API error: {0}")]
    LdkApi(String),
    // #[error("other error: {0}")]
    // Other(String),
}

impl SdkError {
    fn invalid_arg(e: anyhow::Error) -> Self {
        SdkError::InvalidArgument(Self::format_anyhow_error(e))
    }

    fn ldk_api(e: anyhow::Error) -> Self {
        SdkError::LdkApi(Self::format_anyhow_error(e))
    }

    // fn other(e: anyhow::Error) -> Self {
    //     SdkError::Other(Self::format_anyhow_error(e))
    // }

    fn format_anyhow_error(e: anyhow::Error) -> String {
        // Use alternate format (:#) to get the full error chain.
        format!("{:#}", e)
    }
}

pub type Result<T> = std::result::Result<T, SdkError>;

#[derive(Clone, Debug)]
pub struct GetInfoResponse {
    pub pubkey: String,
    // pub alias: String,
    // pub color: String,
    // pub network: String,
    // pub block_height: u32,
}

#[derive(Clone, Debug)]
pub struct MakeInvoiceRequest {
    pub amount_msat: u64,
    // pub description: String,
}

#[derive(Clone, Debug)]
pub struct MakeInvoiceResponse {
    pub bolt11: String,
}

pub struct LdkAlbyClient {
    node: Node<SqliteStore>,
}

pub async fn new_ldk_alby_client(mnemonic: String, work_dir: String) -> Result<Arc<LdkAlbyClient>> {
    let mut builder = Builder::new();
    let mnemonic = Mnemonic::from_str(&mnemonic).unwrap();

    builder.set_entropy_bip39_mnemonic(mnemonic, None);
    builder.set_network(Network::Bitcoin);
    builder.set_esplora_server("https://blockstream.info/api".to_string());
    builder.set_gossip_source_rgs("https://rapidsync.lightningdevkit.org/snapshot".to_string());
    builder.set_storage_dir_path(String::from(
        path::Path::new(&work_dir)
            .join("./storage")
            .to_str()
            .unwrap(),
    ));
    builder.set_log_dir_path(String::from(
        path::Path::new(&work_dir).join("./logs").to_str().unwrap(),
    ));

    let node = builder.build().unwrap();

    node.start().unwrap();

    Ok(Arc::new(LdkAlbyClient { node }))
}

impl LdkAlbyClient {
    pub fn get_info(&self) -> Result<GetInfoResponse> {
        Ok(GetInfoResponse {
            pubkey: self.node.node_id().to_string(),
        })
    }

    pub fn make_invoice(&self, req: MakeInvoiceRequest) -> Result<MakeInvoiceResponse> {
        let invoice = self
            .node
            .receive_payment(req.amount_msat, "Test description", 86400)
            .unwrap();

        Ok(MakeInvoiceResponse {
            bolt11: invoice.to_string(),
        })
    }
}
