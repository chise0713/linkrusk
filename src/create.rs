use chrono::DateTime;
use dioxus::prelude::*;

use crate::{
    utils::{self, CreateRequestBody},
    Route,
};

#[component]
pub fn Create() -> Element {
    rsx! {
        div {
            div { class: "flex flex-col",
                h1 { class: "text-3xl mx-auto", "Create a new link" }
            }
            form {
                class: "mt-5",
                onsubmit: move |evt: FormEvent| async move {
                    let values = evt.values();
                    let url = values["url"].as_value().to_string().into_boxed_str();
                    let length = values["length"].as_value();
                    let length = if length.is_empty() {
                        None
                    } else {
                        match length.parse::<u16>() {
                            Ok(l) => Some(l),
                            Err(e) => {
                                let window = web_sys::window().unwrap();
                                window
                                    .alert_with_message(format!("Invalid length {}", e).as_str())
                                    .unwrap();
                                return;
                            }
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
                        length,
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
                        use_navigator()
                            .push(Route::LinkItem {
                                link: short.into(),
                            });
                    }
                },
                div { class: "flex flex-col w-9/12 sm:w-2/3 mx-auto",
                    div { class: "flex flex-col justify-end mx-auto",
                        div { class: "grid sm:grid-cols-2",
                            div { class: "text-xl", "URL" }
                            div {
                                input {
                                    class: "border border-gray-300 px-2",
                                    r#type: "text",
                                    name: "url",
                                    placeholder: "URL",
                                    required: true,
                                }
                            }
                            div { class: "text-xl", "Length" }
                            div {
                                input {
                                    class: "border border-gray-300 px-2",
                                    r#type: "text",
                                    name: "length",
                                    placeholder: "Length",
                                    value: 6,
                                    required: true,
                                }
                            }
                        }
                        div { class: "grid grid-cols-2",
                            div { class: "text-xl", "Number" }
                            div {
                                input {
                                    r#type: "checkbox",
                                    name: "number",
                                    cursor: "pointer",
                                    placeholder: "Number",
                                    checked: true,
                                }
                            }
                            div { class: "text-xl", "Capital" }
                            div {
                                input {
                                    r#type: "checkbox",
                                    name: "capital",
                                    cursor: "pointer",
                                    checked: true,
                                }
                            }
                            div { class: "text-xl", "Lowercase" }
                            div {
                                input {
                                    r#type: "checkbox",
                                    name: "lowercase",
                                    cursor: "pointer",
                                    checked: true,
                                }
                            }
                        }
                        div { class: "grid sm:grid-cols-2",
                            div { class: "text-xl", "Expiration" }
                            div {
                                input {
                                    class: "border border-gray-300 px-2",
                                    r#type: "datetime",
                                    placeholder: "1970-01-01 00:00:00 UTC",
                                    name: "expiration",
                                }
                            }
                            div { class: "text-xl", "ExpirationTTL" }
                            div {
                                input {
                                    class: "border border-gray-300 px-2",
                                    r#type: "text",
                                    name: "expirationTtl",
                                    placeholder: "Expiration TTL",
                                }
                            }
                        }
                    }
                    div { class: "sm:mt-0 mt-1 mx-auto",
                        button {
                            class: "border border-gray-300 hover:bg-gray-200 px-2 text-2xl",
                            cursor: "pointer",
                            "Create"
                        }
                    }
                }
            }
        }
    }
}
