use chrono::Utc;
use megalodon::entities::Account;

use crate::notification;

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
