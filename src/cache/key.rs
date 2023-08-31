pub fn user_key(username: &String) -> String {
    format!("v2:{}", username)
}

pub fn status_key(instance_url: &String, status_url: &String) -> String {
    format!("v2:{}:{}", instance_url, status_url)
}

pub fn resolve_key(instance_url: &String, status_url: &String) -> String {
    format!("v2:{}:{}:resolve", instance_url, status_url)
}

pub fn follow_key(username: &String) -> String {
    format!("v2:{}:following", username)
}

pub fn sns_key(instance_url: &String) -> String {
    format!("v2:{}:sns", instance_url)
}

pub fn tokens_prefix(worker_id: i32) -> String {
    format!("v2:tokens:worker:{}", worker_id)
}

pub fn token_key(worker_id: i32, token_id: i32) -> String {
    format!("v2:tokens:worker:{}:{}", worker_id, token_id)
}
