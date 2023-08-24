#[test]
fn serializes_sns() {
    let sns = super::SNS::Mastodon;
    let json = serde_json::to_string(&sns);
    assert!(json.is_ok());
    assert_eq!(json.unwrap(), "\"mastodon\"");
}

#[test]
fn deserializes_sns() {
    let json = "\"mastodon\"";
    let sns: Result<super::SNS, serde_json::Error> = serde_json::from_str(json);
    assert!(sns.is_ok());
    assert_eq!(sns.unwrap(), super::SNS::Mastodon);
}

#[test]
fn converts_to_megalodon_sns() {
    let sns = super::SNS::Mastodon;
    let megalodon_sns: megalodon::SNS = sns.into();
    assert_eq!(megalodon_sns, megalodon::SNS::Mastodon);
}

#[test]
fn converts_from_megalodon_sns() {
    let megalodon_sns = megalodon::SNS::Mastodon;
    let sns = super::SNS::from(megalodon_sns);
    assert_eq!(sns, super::SNS::Mastodon);
}
