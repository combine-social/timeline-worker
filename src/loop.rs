use async_std::task;

async fn loop_queue() {
    task::sleep(Duration::new(2, 0)).await;
}

pub async fn run_loop() {
    loop_queue().await;
}
