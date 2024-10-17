use crate::constants::MONGO_DATABASE;
use crate::types::Account;
use mongodb::bson::{doc, Document};
use mongodb::options::{UpdateOneModel, WriteModel};
use mongodb::Collection;

/// Updates all the accounts with the updated balances.
pub async fn update_accounts_balances(accounts: Vec<Account>) -> eyre::Result<()> {
    let db = &*MONGO_DATABASE;
    let client = db.client();
    let relayers: Collection<Document> = db.collection("relayers");

    let mut models = vec![];

    // Build the bulk update query
    for acc in accounts {
        let address = format!("{:#0width$x}", acc.address, width = 64);
        let balance = format!("{:#0width$x}", acc.balance, width = 64);
        models.push(WriteModel::UpdateOne(
            UpdateOneModel::builder()
                .namespace(relayers.namespace())
                .filter(doc! {"address": address})
                .update(doc! { "$set": { "balance": balance } })
                .upsert(true)
                .build(),
        ));
    }

    client.bulk_write(models).await?;
    Ok(())
}
