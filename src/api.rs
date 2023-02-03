use once_cell::sync::Lazy;
//use reqwest::blocking::Client;
use std::collections::HashMap;

const SITE_URL: &str = "https://bilim.integro.kz:8181/processor/back-office/index.faces";
const AUTH_URL: &str = "https://bilim.integro.kz:8181/processor/back-office/j_security_check";

#[allow(non_snake_case)]
pub fn helloWorld() -> String {
    String::from("Hello from Rust! 🦀")
}

static CLIENT_INSTANCE: Lazy<reqwest::blocking::Client> = Lazy::new(|| {
    println!("INIT_CLIENT");
    reqwest::blocking::Client::builder()
        .use_native_tls()
        .cookie_store(true)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
});

pub struct AccessData {
    pub login: String,
    pub password: String,
}

pub fn login(access_data: AccessData) -> String {
    let _ = CLIENT_INSTANCE.get(SITE_URL).send().unwrap();

    let resp = CLIENT_INSTANCE
        .post(AUTH_URL)
        .form(&HashMap::from([
            ("j_username", &access_data.login),
            ("j_password", &access_data.password),
        ]))
        .send()
        .unwrap();

    let text = resp.text().unwrap();
    text
}
