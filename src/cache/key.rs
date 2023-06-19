pub fn status_key(instance_url: &String, status_url: &String) -> String {
    format!("{}:{}", instance_url, status_url)
}

pub fn instance_key(instance_url: &String) -> String {
    instance_url.to_string()
}

fn follow_key(username: &String) -> String {
    format!("{}:following", username)
}
