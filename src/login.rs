use std::rc::Rc;

use dioxus::prelude::*;

#[component]
pub fn LoginForm() -> Element {
    rsx! {
        div { class: "flex justify-center min-h-screen bg-white",
            div { class: "bg-white p-32 text-center",
                h1 { class: "text-3xl", "Login" }
                form {
                    class: "mt-1",
                    onsubmit: move |evt: FormEvent| async move {
                        login_handler(
                                evt.values()["backend_url"].as_value(),
                                evt.values()["token"].as_value(),
                            )
                            .await
                    },
                    input {
                        class: "border border-gray-300 text-xl",
                        r#type: "text",
                        name: "backend_url",
                        placeholder: "Backend Url",
                        required: true,
                    }
                    br {}
                    input {
                        class: "border border-gray-300 text-xl",
                        r#type: "password",
                        name: "token",
                        placeholder: "Token",
                        required: true,
                    }
                    br { class: "mb-2" }
                    button {
                        class: "border border-gray-300 hover:bg-gray-200 text-xl",
                        cursor: "pointer",
                        "Login"
                    }
                }
            }
        }
    }
}

pub async fn check_local_login_info() -> bool {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let url = storage.get_item("backendUrl").unwrap();
    let token = storage.get_item("token").unwrap();
    if url.is_none() || token.is_none() {
        return false;
    }
    check_login_info(url.unwrap(), token.unwrap()).await
}

async fn login_handler(url: impl Into<Rc<str>>, token: impl Into<Rc<str>>) {
    let window = web_sys::window().unwrap();
    let url: Rc<str> = url.into();
    let token: Rc<str> = token.into();
    if url.is_empty() || token.is_empty() {
        window
            .alert_with_message("Please fill in both fields.")
            .unwrap();
        return;
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        window
            .alert_with_message("Please enter a valid URL starting with http:// or https://")
            .unwrap();
        return;
    }
    let url = url.trim_end_matches('/');
    if !check_login_info(url, token.as_ref()).await {
        window
            .alert_with_message("Invalid login information. Please try again.")
            .unwrap();
    } else {
        store_login_info(url, token.as_ref());
        window.alert_with_message("Login successful!").unwrap();
        window.location().reload().unwrap();
    }
}

async fn check_login_info(url: impl Into<Box<str>>, token: impl Into<Box<str>>) -> bool {
    let url: Box<str> = url.into();
    let token: Box<str> = token.into();
    reqwest::Client::new()
        .get(format!("{}/api/v1/list", url))
        .bearer_auth(token)
        .send()
        .await
        .map(|r| r.status() == reqwest::StatusCode::OK)
        .unwrap_or(false)
}

fn store_login_info(url: impl Into<Rc<str>>, token: impl Into<Rc<str>>) {
    let url: Rc<str> = url.into();
    let token: Rc<str> = token.into();
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    storage.set_item("backendUrl", &url).unwrap();
    storage.set_item("token", &token).unwrap();
}
