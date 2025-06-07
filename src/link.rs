use std::rc::Rc;

use chrono::DateTime;
use dioxus::prelude::*;

use crate::{
    utils::{self, Link},
    Route,
};

#[component]
pub fn LinkItem(link: String) -> Element {
    #[allow(clippy::redundant_closure)]
    let links = use_resource(|| utils::fetch_links());
    rsx! {
        match links() {
            Some(links) => {
                let link = links.iter().find(|l| l.short.key.as_ref() == link).cloned();
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
    let expiration = link
        .expiration
        .map(|e| DateTime::from_timestamp(e, 0).unwrap().to_string())
        .unwrap_or_default();
    rsx! {
        div { class: "",
            div { class: "flex flex-col",
                h1 { class: "text-3xl mx-auto", "Key: {key}" }
            }
            form {
                class: "mt-2",
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
                div { class: "flex flex-col w-9/12 sm:w-2/3 mx-auto",
                    div { class: "flex flex-col justify-end mx-auto",
                        div { class: "grid sm:grid-cols-2",
                            span { class: "text-xl", "To URL:" }
                            div {
                                textarea {
                                    class: "border border-gray-300 px-2 break-all",
                                    rows: 4,
                                    resize: "none",
                                    name: "url",
                                    value: "{url}",
                                }
                            }
                            span { class: "text-xl", "Expiration" }
                            div {
                                input {
                                    class: "border border-gray-300 px-2",
                                    r#type: "datetime",
                                    placeholder: "{expiration}",
                                    name: "expiration",
                                }
                            }
                            span { class: "text-xl", "ExpirationTTL" }
                            div {
                                input {
                                    class: "border border-gray-300 px-2",
                                    r#type: "text",
                                    name: "expirationTtl",
                                }
                            }
                        }
                        div { class: "grid sm:grid-cols-2",
                            div { class: "mt-1 sm:mt-0",
                                button {
                                    r#type: "submit",
                                    class: "border border-gray-300 hover:bg-gray-200 px-2 text-2xl",
                                    cursor: "pointer",
                                    "Update"
                                }
                            }
                            div { class: "mt-1 sm:mt-0",
                                button {
                                    r#type: "button",
                                    class: "border border-gray-300 hover:bg-gray-200 px-2 text-2xl  text-red-500",
                                    cursor: "pointer",
                                    onclick: {
                                        let key = key.clone();
                                        move |_| {
                                            let key = key.clone();
                                            async move {
                                                if !web_sys::window()
                                                    .unwrap()
                                                    .confirm_with_message("Are you sure you want to delete this link?")
                                                    .unwrap()
                                                {
                                                    return;
                                                }
                                                if utils::delete_link(key.as_ref()).await {
                                                    let window = web_sys::window().unwrap();
                                                    window.alert_with_message("Link deleted").unwrap();
                                                    use_navigator().replace(Route::Home);
                                                }
                                            }
                                        }
                                    },
                                    "Delete This Link"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
