use megalodon::{
    entities::{Context, Status},
    response::Response,
    SNS,
};
use url::Url;

use super::{client, throttle, throttle::Throttle};

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
    sns: &Option<SNS>,
) -> Result<Option<Context>, megalodon::error::Error> {
    if let Some(status_url) = status.url.clone() {
        let url = Url::parse(status_url.as_str())?;
        if let Some(host) = url.host_str() {
            let rpm = 7500 / 5;
            throttle::throttled(throttle, &host.to_string(), Some(rpm), || async {
                unwrap_context(
                    client::anonymous_client(host, sns.clone())
                        .get_status_context(status.id.clone(), None)
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
