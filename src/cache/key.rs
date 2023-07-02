pub fn status_key(instance_url: &String, status_url: &String) -> String {
    format!("v2:{}:{}", instance_url, status_url)
}

pub fn follow_key(username: &String) -> String {
    format!("v2:{}:following", username)
}
