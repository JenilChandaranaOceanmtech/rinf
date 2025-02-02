//! This crate is written for Rinf demonstrations.

pub use fractal::draw_fractal_image;

mod fractal;

// `machineid_rs` only supports desktop platforms.

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
pub fn get_hardward_id() -> Option<String> {
    let mut builder = machineid_rs::IdBuilder::new(machineid_rs::Encryption::MD5);
    builder
        .add_component(machineid_rs::HWIDComponent::SystemID)
        .add_component(machineid_rs::HWIDComponent::CPUCores);
    let hwid = builder.build("mykey").unwrap();
    Some(hwid)
}
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn get_hardward_id() -> Option<String> {
    None
}

// `chrono` supports all platforms, including web.

use chrono::{offset, DateTime};
pub fn get_current_time() -> DateTime<offset::Local> {
    offset::Local::now()
}

// `reqwest` supports all platforms, including web.

pub async fn fetch_from_web_api(url: &str) -> String {
    reqwest::get(url)
        .await
        .expect("Could not get the response from the example web API.")
        .text()
        .await
        .expect("Could not read body from the web response.")
}
