use std::collections::HashMap;
use std::sync::Arc;

use chrono::NaiveDate;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::Expense;

/// Shared, thread-safe in-memory store. `Arc` so every request handler
/// can hold a cheap clone of the handle; `RwLock` so many readers
/// (GET) can run concurrently while writes (POST/PUT/DELETE) are exclusive.
pub type Db = Arc<RwLock<HashMap<Uuid, Expense>>>;

/// Builds the store and fills it with a few sample rows so the API is
/// immediately useful to poke at — nothing is read from or written to disk.
pub fn seed_db() -> Db {
    let mut map = HashMap::new();

    let seed = [
        ("Team lunch", 42.50, "Food", 2026, 7, 2),
        ("Monitor stand", 89.00, "Office", 2026, 7, 5),
        ("Client dinner", 120.00, "Food", 2026, 7, 10),
        ("Domain renewal", 14.99, "Software", 2026, 7, 12),
        ("Taxi to airport", 35.00, "Travel", 2026, 6, 28),
        ("Conference ticket", 350.00, "Travel", 2026, 6, 15),
    ];

    for (description, amount, category, y, m, d) in seed {
        let id = Uuid::new_v4();
        map.insert(
            id,
            Expense {
                id,
                description: description.to_string(),
                amount,
                category: category.to_string(),
                date: NaiveDate::from_ymd_opt(y, m, d).expect("valid seed date"),
            },
        );
    }

    Arc::new(RwLock::new(map))
}
