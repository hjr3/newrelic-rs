//! Mock for OS X
//!
//! The New Relic SDK is linux only

#![allow(unused_variables)]

pub fn init(license: &str, app_name: &str, language: &str, language_version: &str) {
}

/// Identify the beginning of a transaction.
pub fn transaction_begin() -> Result<i64, ()> {
    Ok(0)
}

/// Set a name for the transaction
///
/// Must be called after `transaction_begin()` and before `transaction_end()`.
pub fn transaction_set_name(transaction_id: i64, name: &str) -> Result<(), ()> {
    Ok(())
}

/// Identify the end of a transaction
pub fn transaction_end(transaction_id: i64) -> Result<(), ()> {
    Ok(())
}

/// Identify the beginning of a segment that performs a generic operation.
///
/// This type of segment does not create metrics, but can show up in a
/// transaction trace if a transaction is slow enough.
pub fn segment_generic_begin(transaction_id: i64, name: &str) -> Result<i64, ()> {
    Ok(0)
}

/// Identify the beginning of a segment that performs an external service.
pub fn segment_external_begin(transaction_id: i64, host: &str, name: &str) -> Result<i64, ()> {
    Ok(0)
}

/// Identify the end of a segment
pub fn segment_end(transaction_id: i64, segment_id: i64) -> Result<(), ()> {
    Ok(())
}
