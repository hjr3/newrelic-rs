#[macro_use] extern crate log;


#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::{
    init,
    transaction_begin,
    transaction_set_name,
    transaction_end,
    segment_generic_begin,
    segment_external_begin,
    segment_end,
};

#[cfg(not(target_os = "linux"))]
mod macos;

#[cfg(not(target_os = "linux"))]
pub use self::macos::{
    init,
    transaction_begin,
    transaction_set_name,
    transaction_end,
    segment_generic_begin,
    segment_external_begin,
    segment_end,
};

pub fn transaction_with<F, T>(f: F, name: Option<&str>) -> T where F: Fn(Option<i64>) -> T {
    let transaction_id = self::transaction_begin();
    let r = f(transaction_id.ok());
    if transaction_id.is_ok() {

        if name.is_some() {
            self::transaction_set_name(transaction_id.unwrap(), name.unwrap()).unwrap_or_else(|_| {

                error!("Failed to name New Relic transaction");
            });
        }

        self::transaction_end(transaction_id.unwrap()).unwrap_or_else(|_| {

            error!("Failed to end New Relic transaction");
        });
    } else {
        error!("Failed to start New Relic transaction");
    }

    r
}

pub fn segment_generic_with<F, T>(f: F, transaction_id: Option<i64>, name: &str) -> T where F: Fn() -> T {

    let segment_id = if transaction_id.is_some() {
        self::segment_generic_begin(transaction_id.unwrap(), name)
    } else {
        Err(())
    };

    let r = f();

    if segment_id.is_ok() {
        self::segment_end(transaction_id.unwrap(), segment_id.unwrap()).unwrap_or_else(|_| {
            warn!("Failed to finish tracking New Relic generic segment");
        });
    }

    r
}

pub fn segment_external_with<F, T>(f: F, transaction_id: Option<i64>, host: &str, name: &str) -> T where F: Fn() -> T {

    let segment_id = if transaction_id.is_some() {
        self::segment_external_begin(transaction_id.unwrap(), host, name)
    } else {
        Err(())
    };

    let r = f();

    if segment_id.is_ok() {
        self::segment_end(transaction_id.unwrap(), segment_id.unwrap()).unwrap_or_else(|_| {
            warn!("Failed to finish tracking New Relic generic segment");
        });
    }

    r
}
