pub mod activitypub;

#[derive(Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub(crate) max_id: Option<String>,
}
