use megalodon::{
    entities::{Context, Status},
    megalodon::GetStatusContextInputOptions,
    response::Response,
    SNS,
};
use url::Url;

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
    status: &Status,
    throttle: &mut Throttle,
    sns: Option<&SNS>,
) -> Result<Option<Context>, megalodon::error::Error> {
    if let Some(status_url) = status.url.clone() {
        let url = Url::parse(status_url.as_str())?;
        if let Some(host) = url.host_str() {
            let rpm = 7500 / 5;
            throttle::throttled(throttle, &host.to_string(), Some(rpm), || async {
                unwrap_context(
                    client::anonymous_client(host, sns.cloned())
                        .get_status_context(status.id.clone(), context_options())
                        .await,
                )
            })
            .await
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
