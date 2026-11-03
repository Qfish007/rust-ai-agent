use std::sync::OnceLock;

use tokio::sync::Semaphore;

// Semaphore 信号量，用于控制并发请求的数量
static SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();

pub fn get_semaphore() -> &'static Semaphore {
    SEMAPHORE.get_or_init(|| Semaphore::new(3))
}
