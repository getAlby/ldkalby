use std::sync::Arc;

use once_cell::sync::Lazy;

mod ldk_alby_client;
use ldk_alby_client::{new_ldk_alby_client, LdkAlbyClient, Result, SdkError};

pub use ldk_alby_client::{GetInfoResponse, MakeInvoiceRequest, MakeInvoiceResponse};

static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

pub struct BlockingLdkAlbyClient {
    ldk_alby_client: Arc<LdkAlbyClient>,
}

// TODO: is a blocking client still needed?
impl BlockingLdkAlbyClient {
    pub fn get_info(&self) -> Result<GetInfoResponse> {
        self.ldk_alby_client.get_info()
    }
    pub fn make_invoice(&self, req: MakeInvoiceRequest) -> Result<MakeInvoiceResponse> {
        self.ldk_alby_client.make_invoice(req)
    }
}

pub fn new_blocking_ldk_alby_client(
    mnemonic: String,
    work_dir: String,
) -> Result<Arc<BlockingLdkAlbyClient>> {
    rt().block_on(async move {
        let ldk_alby_client = new_ldk_alby_client(mnemonic, work_dir).await?;
        let blocking_ldk_alby_client = Arc::new(BlockingLdkAlbyClient { ldk_alby_client });

        Ok(blocking_ldk_alby_client)
    })
}

fn rt() -> &'static tokio::runtime::Runtime {
    &RT
}

uniffi::include_scaffolding!("ldkalby");
