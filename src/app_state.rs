extern crate rocket;

use mysql as my;
use serde_json::Value;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<my::Pool>,
    pub config: Value,
    threads_running: Arc<Mutex<i64>>,
    shutting_down: Arc<Mutex<bool>>,
}

impl AppState {
    pub fn new_from_config(config: &Value) -> Self {
        Self {
            db: Arc::new(AppState::db_pool_from_config(config)),
            config: config.to_owned(),
            threads_running: Arc::new(Mutex::new(0)),
            shutting_down: Arc::new(Mutex::new(false)),
        }
    }

    fn db_pool_from_config(config: &Value) -> my::Pool {
        let mut builder = my::OptsBuilder::new();
        //println!("{}", &self.params);
        builder
            .ip_or_hostname(config["host"].as_str())
            .db_name(config["schema"].as_str())
            .user(config["user"].as_str())
            .pass(config["password"].as_str());
        builder.tcp_port(config["db_port"].as_u64().unwrap_or(3306) as u16);

        // Min 1, max 7 connections
        match my::Pool::new_manual(1, 7, builder) {
            Ok(pool) => pool,
            Err(e) => panic!("Could not initialize DB connection pool: {}", &e),
        }
    }

    pub fn modify_threads_running(&self, diff: i64) {
        let mut threads_running = self.threads_running.lock().unwrap();
        *threads_running += diff;
        if self.is_shutting_down() && *threads_running == 0 {
            panic!("Planned shutdown")
        }
    }

    pub fn is_shutting_down(&self) -> bool {
        *self.shutting_down.lock().unwrap()
    }
}
