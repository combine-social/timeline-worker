pub struct Connection {
    pub taken: bool,
}
pub struct ConnectionPool {}

pub async fn create_pool() -> Result<ConnectionPool, sqlx::Error> {
    Ok(ConnectionPool {})
}

pub async fn connect(_pool: &ConnectionPool) -> Result<Connection, sqlx::Error> {
    Ok(Connection { taken: false })
}
