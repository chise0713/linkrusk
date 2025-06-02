use chrono::DateTime;
use dioxus::prelude::*;

use crate::utils::{self, CreateRequestBody};

#[component]
pub fn Create() -> Element {
    rsx! {
        div {
            h1 { class: "text-3xl px-5", "Create a new link" }
            form {
                class: "ml-12 mt-5",
                onsubmit: move |evt: FormEvent| async move {
                    let values = evt.values();
                    let url = values["url"].as_value().to_string().into_boxed_str();
                    let length = match values["length"].as_value().parse() {
                        Ok(l) => l,
                        Err(e) => {
                            let window = web_sys::window().unwrap();
                            window
                                .alert_with_message(format!("Invalid length {}", e).as_str())
                                .unwrap();
                            0
                        }
                    };
                    let number = values["number"].as_value().parse::<bool>().ok();
                    let capital = values["capital"].as_value().parse::<bool>().ok();
                    let lowercase = values["lowercase"].as_value().parse::<bool>().ok();
                    let expiration = values["expiration"].as_value();
                    let expiration = if expiration.is_empty() {
                        None
                    } else {
                        match DateTime::parse_from_str(&expiration, "%Y-%m-%d %H:%M:%S %z") {
                            Ok(d) => Some(d.timestamp()),
                            Err(e) => {
                                web_sys::window()
                                    .unwrap()
                                    .alert_with_message(
                                        &format!("Invalid expiration date format, {}", e),
                                    )
                                    .unwrap();
                                return;
                            }
                        }
                    };
                    let expiration_ttl = values["expirationTtl"].as_value();
                    let expiration_ttl = if expiration_ttl.is_empty() {
                        None
                    } else {
                        match expiration_ttl.parse::<u32>() {
                            Ok(e) => Some(e),
                            Err(e) => {
                                web_sys::window()
                                    .unwrap()
                                    .alert_with_message(&format!("Invalid expirationTTL, {}", e))
                                    .unwrap();
                                return;
                            }
                        }
                    };
                    let body = CreateRequestBody {
                        url,
                        length: Some(length.to_string().into_boxed_str()),
                        number,
                        capital,
                        lowercase,
                        expiration,
                        expiration_ttl,
                    };
                    let short = utils::create_link(body).await;
                    let window = web_sys::window().unwrap();
                    if !short.is_empty() {
                        window
                            .alert_with_message(format!("Link created: {short}").as_str())
                            .unwrap();
                        window.location().set_href(&format!("/link/{short}")).unwrap();
                    }
                },
                span { class: "pl-5 text-xl pr-25", "URL" }
                input {
                    class: "border border-gray-300 pl-2",
                    r#type: "text",
                    name: "url",
                    placeholder: "URL",
                    required: true,
                }
                br {}
                span { class: "pl-5 text-xl pr-18.5", "Length" }
                input {
                    class: "border border-gray-300 pl-2",
                    r#type: "text",
                    name: "length",
                    placeholder: "Length",
                    value: 6,
                    required: true,
                }
                br {}
                span { class: "pl-5 text-xl pr-19", "Number" }
                input {
                    r#type: "checkbox",
                    name: "number",
                    cursor: "pointer",
                    placeholder: "Number",
                    checked: true,
                }
                br {}
                span { class: "pl-5 text-xl pr-21.25", "Capital" }
                input {
                    r#type: "checkbox",
                    name: "capital",
                    cursor: "pointer",
                    checked: true,
                }
                br {}
                span { class: "pl-5 text-xl pr-12.75", "Lowercase" }
                input {
                    r#type: "checkbox",
                    name: "lowercase",
                    cursor: "pointer",
                    checked: true,
                }
                br {}
                span { class: "pl-5 text-xl pr-11.25", "Expiration" }
                input {
                    class: "border border-gray-300 pl-2 pr-2",
                    r#type: "datetime",
                    placeholder: "1970-01-01 00:00:00 UTC",
                    name: "expiration",
                }
                br {}
                span { class: "pl-5 text-xl pr-2.5", "ExpirationTTL" }
                input {
                    class: "border border-gray-300 pl-2 pr-2",
                    r#type: "text",
                    name: "expirationTtl",
                    placeholder: "Expiration TTL",
                }
                br {}
                span { class: "pl-5" }
                button {
                    class: "border border-gray-300 hover:bg-gray-200 px-2 text-2xl",
                    cursor: "pointer",
                    "Create"
                }
            }
        }
    }
}
