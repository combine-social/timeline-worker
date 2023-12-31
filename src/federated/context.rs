use megalodon::{entities::Context, megalodon::GetStatusContextInputOptions, response::Response};

use crate::strerr::here;

use super::{client, throttle};

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
) -> Result<Option<Context>, String> {
    let rpm = 7500 / 5;
    info!("throttled call to get_status_context");
    let result = throttle::throttled(instance_url, Some(rpm), || async {
        unwrap_context(
            client::anonymous_client(instance_url)
                .await
                .get_status_context(status_id.to_owned(), context_options())
                .await,
        )
    })
    .await;
    if let Err(error) = result {
        match error {
            megalodon::error::Error::JsonError(err) => {
                warn!(
                    "JSON parse error for {}#{}: {:?}",
                    instance_url, status_id, err
                );
                Ok(None)
            }
            megalodon::error::Error::ParseError(err) => {
                warn!("Parse error for {}#{}: {:?}", instance_url, status_id, err);
                Ok(None)
            }
            megalodon::error::Error::OwnError(ref own_err) => {
                if let Some(status) = own_err.status {
                    if status == 401 || status == 403 {
                        warn!(
                            "Authentication required for {}#{}, ignoring",
                            instance_url, status_id
                        );
                        Ok(None)
                    } else if status >= 404 {
                        warn!(
                            "Status context not found for {}#{}, ignoring",
                            instance_url, status_id
                        );
                        Ok(None)
                    } else if status >= 500 {
                        warn!(
                            "Internal server for {}#{}, ignoring",
                            instance_url, status_id
                        );
                        Ok(None)
                    } else {
                        Err(here!(error))
                    }
                } else {
                    Err(here!(error))
                }
            }
            megalodon::error::Error::RequestError(error) => {
                error!(
                    "Failed making context request for {:?}#{:?}: {:?}",
                    instance_url, status_id, error
                );
                Ok(None)
            }
            _ => {
                error!(
                    "Failed getting context for {:?}#{:?}: {:?}",
                    instance_url, status_id, error
                );
                Ok(None)
            }
        }
    } else {
        result.map_err(|err| here!(err))
    }
}
