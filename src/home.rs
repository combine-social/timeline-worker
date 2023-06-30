use crate::{
    cache::Cache,
    federated::{self},
    queue_statuses,
    repository::tokens::Token,
};

pub async fn queue_home_statuses(token: &Token, cache: &mut Cache) -> Result<(), String> {
    queue_statuses::queue_statuses(token, cache, |max_id| async move {
        federated::get_home_timeline_page(token, max_id).await
    })
    .await
}
