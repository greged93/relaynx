use relaynx::balances;
use relaynx::constants::{BALANCES_QUERY_INTERVAL, RELAYERS_ADDRESS};
use relaynx::database::update_accounts_balances;
use relaynx::types::Account;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load environment variables from .env file.
    dotenvy::dotenv()?;
    setup_tracing()?;
    let relayers = RELAYERS_ADDRESS.clone();

    // Start a task that indexes the balances every x seconds
    let handle = tokio::spawn(async move {
        loop {
            let maybe_balances = balances(&relayers).await;
            if maybe_balances.is_err() {
                tracing::error!(target: "indexer", err = ?maybe_balances.unwrap_err(), "failed to fetch balances");
                continue;
            }
            let balances = maybe_balances.expect("not error");

            // Update the balance of the accounts in the database
            let accounts = relayers
                .iter()
                .zip(balances.iter())
                .map(|(add, bal)| Account::new(*add, *bal))
                .collect::<Vec<_>>();
            let _ = update_accounts_balances(accounts)
                .await
                .inspect_err(|err| tracing::error!(target: "indexer", ?err));

            tracing::info!(target: "indexer", ?balances);
            tokio::time::sleep(tokio::time::Duration::from_secs(BALANCES_QUERY_INTERVAL)).await;
        }
    });

    handle.await?;
    Ok(())
}

/// Sets up the tracing.
fn setup_tracing() -> eyre::Result<()> {
    let filter = EnvFilter::builder().from_env()?;
    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
