use crate::{
    federated::{self},
    queue_statuses,
    repository::tokens::Token,
};

pub async fn queue_home_statuses(token: &Token) -> Result<(), String> {
    queue_statuses::queue_statuses(token, |max_id| async move {
        federated::get_home_timeline_page(token, max_id).await
    })
    .await
}
