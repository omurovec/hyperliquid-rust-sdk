#![deny(unreachable_pub)]
mod consts;
mod errors;
pub mod exchange;
mod helpers;
mod info;
mod market_maker;
mod meta;
mod prelude;
mod proxy_digest;
mod req;
mod signature;
mod ws;
pub use consts::{EPSILON, LOCAL_API_URL, MAINNET_API_URL, TESTNET_API_URL};
pub use errors::Error;
pub use exchange::*;
pub use helpers::{bps_diff, truncate_float, BaseUrl};
pub use info::{info_client::*, *};
pub use market_maker::{MarketMaker, MarketMakerInput, MarketMakerRestingOrder};
pub use meta::{AssetMeta, Meta};
pub use ws::*;

// make signature module public
pub mod pub_signature {
    use ethers::{
        signers::LocalWallet,
        types::{Signature, H256},
    };

    use crate::prelude::Result;

    // Public wrapper around the internal function
    pub fn create_l1_signature(
        wallet: &LocalWallet,
        connection_id: H256,
        is_mainnet: bool,
    ) -> Result<Signature> {
        // Call the internal function
        super::signature::sign_l1_action(wallet, connection_id, is_mainnet)
    }

    // If you need the generic sign_typed_data function
    pub fn create_typed_signature<T: ethers::types::transaction::eip712::Eip712>(
        payload: &T,
        wallet: &LocalWallet,
    ) -> Result<Signature> {
        super::signature::sign_typed_data(payload, wallet)
    }
}
