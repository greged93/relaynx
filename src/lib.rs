use crate::constants::{ERC20Reader, PROVIDER, STARKNET_NATIVE_TOKEN};
use starknet::core::types::requests::CallRequest;
use starknet::core::types::{BlockId, BlockTag, Felt};
use starknet::providers::{Provider, ProviderRequestData, ProviderResponseData};
use tracing::instrument;

pub mod constants;
pub mod database;
pub mod types;

/// Returns the balance of the given addresses.
#[instrument(skip_all)]
pub async fn balances(addresses: &[Felt]) -> eyre::Result<Vec<Felt>> {
    let contract = ERC20Reader::new(*STARKNET_NATIVE_TOKEN, &*PROVIDER);
    let requests = addresses
        .iter()
        .map(|add| contract.balanceOf(add).call_raw)
        .map(|request| {
            ProviderRequestData::Call(CallRequest {
                request,
                block_id: BlockId::Tag(BlockTag::Latest),
            })
        })
        .collect::<Vec<_>>();

    let balances = PROVIDER
        .batch_requests(requests)
        .await?
        .into_iter()
        .filter_map(|res| {
            if let ProviderResponseData::Call(res) = res {
                Some(res.first().copied()?)
            } else {
                // This should never happen
                tracing::error!("incorrect provider response variant");
                None
            }
        })
        .collect();

    Ok(balances)
}
