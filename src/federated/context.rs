use megalodon::{
    entities::Context, megalodon::GetStatusContextInputOptions, response::Response, SNS,
};

use super::{client, throttle, throttle::Throttle};

fn context_options() -> Option<&'static GetStatusContextInputOptions> {
    Some(&GetStatusContextInputOptions {
        limit: Some(25),
        max_id: None,
        since_id: None,
    })
}

fn unwrap_context(
    result: Result<Response<Context>, megalodon::error::Error>,
) -> Result<Option<Context>, megalodon::error::Error> {
    if let Ok(response) = result {
        Ok(Some(response.json))
    } else {
        Err(result.err().unwrap())
    }
}
pub async fn get_context(
    instance_url: &String,
    status_id: &str,
    throttle: &mut Throttle,
    sns: Option<&SNS>,
) -> Result<Option<Context>, String> {
    let rpm = 7500 / 5;
    throttle::throttled(throttle, instance_url, Some(rpm), || async {
        unwrap_context(
            client::anonymous_client(instance_url, sns.cloned())
                .get_status_context(status_id.to_owned(), context_options())
                .await,
        )
    })
    .await
    .map_err(|error| error.to_string())
}
