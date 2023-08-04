use megalodon::entities::Status;

use crate::strerr::here;

pub trait OriginId {
    fn origin_id(&self) -> Result<String, String>;
}

impl OriginId for Status {
    fn origin_id(&self) -> Result<String, String> {
        self.uri
            .clone()
            .split('/')
            .last()
            .map(|s| Ok(s.to_owned()))
            .unwrap_or_else(|| {
                let msg = format!("Bad uri format for {}: {}", &self.id, &self.uri);
                Err(here!(msg))
            })
    }
}
