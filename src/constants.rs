use cainome::rs::abigen_legacy;
use mongodb::options::{DatabaseOptions, ReadConcern, WriteConcern};
use mongodb::Database;
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Url};
use std::iter::Iterator;
use std::str::FromStr;
use std::sync::LazyLock;

/// The delay time in seconds between balances queries
pub const BALANCES_QUERY_INTERVAL: u64 = 15;

/// Starknet native token address
pub static STARKNET_NATIVE_TOKEN: LazyLock<Felt> = LazyLock::new(|| {
    Felt::from_hex("0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap()
});

/// Relayer addresses
pub static RELAYERS_ADDRESS: LazyLock<Vec<Felt>> = LazyLock::new(|| {
    std::env::var("RELAYERS_ADDRESSES")
        .expect("missing RELAYERS_ADDRESSES")
        .split(',')
        .filter_map(|x| Felt::from_str(x).ok())
        .collect()
});

/// The provider to the starknet network.
pub static PROVIDER: LazyLock<JsonRpcClient<HttpTransport>> = LazyLock::new(|| {
    let url = std::env::var("PROVIDER_URL").expect("missing PROVIDER_URL");
    JsonRpcClient::new(HttpTransport::new(
        Url::parse(&url).expect("failed to parse provider url"),
    ))
});

pub static MONGO_DATABASE: LazyLock<Database> = LazyLock::new(|| {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let db_client = mongodb::Client::with_uri_str(
                std::env::var("MONGO_CONNECTION_STRING")
                    .expect("Missing MONGO_CONNECTION_STRING .env"),
            )
            .await
            .expect("failed to get mongo client");

            db_client.database_with_options(
                &std::env::var("MONGO_DATABASE_NAME")
                    .expect("Missing MONGO_DATABASE_NAME from .env"),
                DatabaseOptions::builder()
                    .read_concern(ReadConcern::majority())
                    .write_concern(WriteConcern::majority())
                    .build(),
            )
        })
    })
});

abigen_legacy!(
    ERC20,
    r#"[
        {
          "members": [
            {
              "name": "low",
              "offset": 0,
              "type": "felt"
            },
            {
              "name": "high",
              "offset": 1,
              "type": "felt"
            }
          ],
          "name": "Uint256",
          "size": 2,
          "type": "struct"
        },
        {
          "inputs": [
            {
              "name": "account",
              "type": "felt"
            }
          ],
          "name": "balanceOf",
          "outputs": [
            {
              "name": "balance",
              "type": "Uint256"
            }
          ],
          "stateMutability": "view",
          "type": "function"
        }
    ]"#
);
