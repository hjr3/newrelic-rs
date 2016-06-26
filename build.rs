//! Compiler setup for New Relic SDK
//!
//! See http://doc.crates.io/build-script.html for details

#[cfg(target_os = "linux")]
fn build() {
    println!("cargo:rustc-link-search=native=/usr/local/nr_agent_sdk/lib");
    println!("cargo:rustc-link-lib=dylib=newrelic-common");
    println!("cargo:rustc-link-lib=dylib=newrelic-transaction");
    println!("cargo:rustc-link-lib=dylib=newrelic-collector-client");
}

#[cfg(target_os = "macos")]
fn build() {
}

fn main() {
    build();
}
