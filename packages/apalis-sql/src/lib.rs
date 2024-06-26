#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # apalis-sql
//! apalis offers Sqlite, Mysql and Postgres storages for its workers.
//! See relevant modules for examples

use std::time::Duration;

/// The context of the sql job
pub mod context;
/// Util for fetching rows
pub mod from_row;

/// Postgres storage for apalis. Uses `NOTIFY` and `SKIP LOCKED`
#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres;

/// Sqlite Storage for apalis.
/// Uses a transaction and min(rowid)
#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
pub mod sqlite;

/// MySQL storage for apalis. Uses `SKIP LOCKED`
#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
pub mod mysql;

/// Config for sql storages
#[derive(Debug, Clone)]
pub struct Config {
    keep_alive: Duration,
    buffer_size: usize,
    poll_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keep_alive: Duration::from_secs(30),
            buffer_size: 10,
            poll_interval: Duration::from_millis(50),
        }
    }
}

impl Config {
    /// Interval between database poll queries
    ///
    /// Defaults to 30ms
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Interval between worker keep-alive database updates
    ///
    /// Defaults to 30s
    pub fn keep_alive(mut self, keep_alive: Duration) -> Self {
        self.keep_alive = keep_alive;
        self
    }

    /// Buffer size to use when querying for jobs
    ///
    /// Defaults to 10
    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }
}
