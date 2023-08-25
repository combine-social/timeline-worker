use chrono::Utc;
use futures_util::StreamExt;
use megalodon::entities::{
    notification::NotificationType, Account, Notification, Status, StatusVisibility,
};

use crate::{
    federated::test_set_mock_response,
    home,
    models::ContextRequest,
    notification, queue,
    repository::{self, tokens},
};

#[tokio::test]
async fn queues_a_home_timeline_status() {
    let db = repository::create_pool().await.unwrap();
    let mut connection = repository::connect(&db).await.unwrap();
    let mut tokens = tokens::find_by_worker_id(&mut connection, 1);
    let token = tokens.next().await.unwrap();
    let queue_name = &token.username;
    test_set_mock_response(&vec![Status {
        id: "status_id".to_owned(),
        uri: "https://example.com/status_id".to_owned(),
        url: Some("https://example.com/status_id".to_owned()),
        account: Account {
            id: "account_id".to_owned(),
            username: "@username".to_owned(),
            acct: "username".to_owned(),
            display_name: "John Doe".to_owned(),
            locked: false,
            discoverable: None,
            group: None,
            noindex: None,
            suspended: None,
            limited: None,
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
            source: None,
            role: None,
            mute_expires_at: None,
            fields: vec![],
            bot: false,
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
    _ = home::queue_home_statuses(&token).await;
    let queue_result: Result<Option<ContextRequest>, String> = queue::next(queue_name).await;
    assert!(queue_result.is_ok());
    let context = queue_result.unwrap().unwrap();
    assert_eq!(context.instance_url, "example.com".to_owned());
    assert_eq!(context.status_id, "status_id".to_owned());
}

#[tokio::test]
async fn queues_a_notification_timeline_status() {
    let db = repository::create_pool().await.unwrap();
    let mut connection = repository::connect(&db).await.unwrap();
    let mut tokens = tokens::find_by_worker_id(&mut connection, 1);
    let token = tokens.next().await.unwrap();
    let queue_name = &token.username;
    test_set_mock_response(&vec![Notification {
        account: Account {
            id: "account_id".to_owned(),
            username: "@username".to_owned(),
            acct: "username".to_owned(),
            display_name: "John Doe".to_owned(),
            locked: false,
            discoverable: None,
            group: None,
            noindex: None,
            suspended: None,
            limited: None,
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
            source: None,
            role: None,
            mute_expires_at: None,
            fields: vec![],
            bot: false,
        },
        created_at: Utc::now(),
        id: "n_id_1".to_owned(),
        status: Some(Status {
            id: "status_id".to_owned(),
            uri: "https://example.com/status_id".to_owned(),
            url: Some("https://example.com/status_id".to_owned()),
            account: Account {
                id: "account_id".to_owned(),
                username: "@username".to_owned(),
                acct: "username".to_owned(),
                display_name: "John Doe".to_owned(),
                locked: false,
                discoverable: None,
                group: None,
                noindex: None,
                suspended: None,
                limited: None,
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
                source: None,
                role: None,
                mute_expires_at: None,
                fields: vec![],
                bot: false,
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
        }),
        emoji: None,
        target: None,
        r#type: NotificationType::Status,
    }]);
    _ = home::queue_home_statuses(&token).await;
    let queue_result: Result<Option<ContextRequest>, String> = queue::next(queue_name).await;
    assert!(queue_result.is_ok());
    let context = queue_result.unwrap().unwrap();
    assert_eq!(context.instance_url, "example.com".to_owned());
    assert_eq!(context.status_id, "status_id".to_owned());
}

#[test]
fn gets_user_at_hostname_acct() {
    let account = Account {
        id: "account_id".to_owned(),
        username: "@username".to_owned(),
        acct: "username".to_owned(),
        display_name: "John Doe".to_owned(),
        locked: false,
        discoverable: None,
        group: None,
        noindex: None,
        suspended: None,
        limited: None,
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
        source: None,
        role: None,
        mute_expires_at: None,
        fields: vec![],
        bot: false,
    };
    let result = notification::acct(&account);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "username@example.com".to_owned());
}
