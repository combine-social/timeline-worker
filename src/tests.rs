use chrono::Utc;
use futures_util::StreamExt;
use megalodon::entities::{Account, Status, StatusVisibility};

use crate::{
    cache,
    federated::{self, test_set_mock_response},
    home,
    models::ContextRequest,
    queue,
    repository::{self, tokens},
};

#[tokio::test]
async fn queues_a_home_timeline_status() {
    let db = repository::create_pool().await.unwrap();
    let mut connection = repository::connect(&db).await.unwrap();
    let mut cache = cache::connect().await.unwrap();
    let mut throttle = federated::throttle::initialize();
    let mut tokens = tokens::find_by_worker_id(&mut connection, 1);
    let token = tokens.next().await.unwrap();
    let queue_name = &token.username;
    test_set_mock_response(&vec![Status {
        id: "status_id".to_owned(),
        uri: "https://example.com/status_id".to_owned(),
        url: Some("https:/Some(/Some(example.com/status_i))d".to_owned()),
        account: Account {
            id: "account_id".to_owned(),
            username: "@username".to_owned(),
            acct: "https://example.com/username".to_owned(),
            display_name: "John Doe".to_owned(),
            locked: false,
            discoverable: None,
            group: None,
            created_at: Utc::now(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 1,
            note: "".to_owned(),
            url: "https://example.com/account_id".to_owned(),
            avatar: "https://example.com/account/avatar.png".to_owned(),
            avatar_static: "https://example.com/account/avatar.png".to_owned(),
            header: "".to_owned(),
            header_static: "".to_owned(),
            emojis: vec![],
            moved: None,
            fields: vec![],
            bot: false,
            source: None,
        },
        in_reply_to_id: None,
        in_reply_to_account_id: None,
        reblog: None,
        content: "<p>a status</p>".to_owned(),
        plain_content: None,
        created_at: Utc::now(),
        emojis: vec![],
        replies_count: 0,
        reblogs_count: 0,
        favourites_count: 0,
        reblogged: None,
        favourited: None,
        muted: None,
        sensitive: false,
        spoiler_text: "".to_owned(),
        visibility: StatusVisibility::Public,
        media_attachments: vec![],
        mentions: vec![],
        tags: vec![],
        card: None,
        poll: None,
        application: None,
        language: None,
        pinned: None,
        emoji_reactions: None,
        quote: false,
        bookmarked: None,
    }]);
    _ = home::queue_home_statuses(&token, &mut cache, &mut throttle).await;
    let queue_result: Result<Option<ContextRequest>, String> = queue::next(queue_name).await;
    assert!(queue_result.is_ok());
    assert!(queue_result.ok().is_some_and(|x| x.is_some()));
}
