use std::rc::Rc;

use chrono::DateTime;
use dioxus::prelude::*;

use crate::utils::{self, Link};

#[component]
pub fn LinkItem(link: String) -> Element {
    let links = use_resource(|| utils::fetch_links());
    rsx! {
        match links() {
            Some(links) => {
                let link = links.iter().find(|l| l.short.key.as_ref() == &link).cloned();
                if let Some(link) = link {
                    link_item_render(link)
                } else {
                    rsx! {
                        p { class: "mb-2 text-2xl", "Link not found." }
                        "You might need a refresh if you came from create"
                    }
                }
            }
            None => rsx! {
                div { class: "mb-2 text-2xl", "Loading..." }
            },
        }
    }
}

fn link_item_render(link: Link) -> Element {
    let key: Rc<str> = link.short.key.clone().into();
    let url = link
        .url
        .clone()
        .unwrap_or_else(|| Box::from("Failed to display"));
    rsx! {
        div { class: "p-4",
            p { class: "mb-2 text-2xl", "Key: {key}" }
            form {
                onsubmit: {
                    let key = key.clone();
                    move |event: FormEvent| {
                        let key = key.clone();
                        async move {
                            let values = event.values();
                            let url = values["url"].as_value().to_string().into_boxed_str();
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
                                            .alert_with_message(
                                                &format!("Invalid expirationTTL, {}", e),
                                            )
                                            .unwrap();
                                        return;
                                    }
                                }
                            };
                            let body = utils::UpdateRequestBody {
                                short: key.as_ref().into(),
                                url,
                                expiration,
                                expiration_ttl,
                            };
                            if utils::update_link(body).await {
                                let window = web_sys::window().unwrap();
                                window.alert_with_message("Link updated").unwrap();
                                window.location().reload().unwrap();
                            }
                        }
                    }
                },
                span { class: "pl-5 text-xl pr-16.5", "To URL:" }
                input {
                    class: "w-10/12 border border-gray-300",
                    r#type: "text",
                    name: "url",
                    value: "{url}",
                }
                span { class: "px-1" }
                br {}
                span { class: "pl-5 text-xl pr-11.25", "Expiration" }
                input {
                    class: "border border-gray-300 pl-2 pr-2",
                    r#type: "datetime",
                    placeholder: "YYYY-MM-DD HH:MM:SS +0000",
                    name: "expiration",
                }
                br {}
                span { class: "pl-5 text-xl pr-2.5", "ExpirationTTL" }
                input {
                    class: "border border-gray-300 pl-2 pr-2",
                    r#type: "text",
                    name: "expirationTtl",
                }
                br {}
                span { class: "pl-5 " }
                button {
                    class: "border border-gray-300 hover:bg-gray-200 px-2 text-2xl",
                    cursor: "pointer",
                    "Update"
                }
            }
            br {}
            span { class: "pl-5" }
            button {
                class: "border border-gray-300 hover:bg-gray-200 px-2 text-red-500",
                cursor: "pointer",
                onclick: {
                    let key = key.clone();
                    move |_| {
                        let key = key.clone();
                        async move {
                            if utils::delete_link(key.as_ref()).await {
                                let window = web_sys::window().unwrap();
                                window.alert_with_message("Link deleted").unwrap();
                                window.location().set_href("/").unwrap();
                            }
                        }
                    }
                },
                "Delete This Link"
            }
        }
    }
}
