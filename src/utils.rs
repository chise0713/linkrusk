use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub ok: bool,
    pub msg: Box<str>,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListData {
    pub cursor: Option<Box<str>>,
    pub list_complete: bool,
    pub links: Box<[Link]>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Link {
    pub short: Short,
    pub url: Option<Box<str>>,
    pub expiration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Short {
    pub key: Box<str>,
    #[serde(rename = "noHttps")]
    pub no_https: Box<str>,
    pub full: Box<str>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateRequestBody {
    pub url: Box<str>,
    pub length: Option<Box<str>>,
    pub number: Option<bool>,
    pub capital: Option<bool>,
    pub lowercase: Option<bool>,
    pub expiration: Option<i64>,
    pub expiration_ttl: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateData {
    pub short: Box<str>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateRequestBody {
    pub short: Box<str>,
    pub url: Box<str>,
    pub expiration: Option<i64>,
    pub expiration_ttl: Option<u32>,
}

fn get_login_info() -> Option<(Box<str>, Box<str>)> {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let url = match storage.get_item("backendUrl").unwrap() {
        Some(url) => url.into_boxed_str(),
        None => {
            storage.clear().unwrap();
            window
                .alert_with_message("Failed to fetch the backend URL. Please login again.")
                .unwrap();
            window.location().set_href("/").unwrap();
            return None;
        }
    };
    let token = match storage.get_item("token").unwrap() {
        Some(token) => token.into_boxed_str(),
        None => {
            storage.clear().unwrap();
            window
                .alert_with_message("Failed to fetch the token. Please login again.")
                .unwrap();
            window.location().set_href("/").unwrap();
            return None;
        }
    };
    Some((url, token))
}

pub async fn fetch_links() -> Box<[Link]> {
    let null_link = Box::from([]);
    let (url, token) = match get_login_info() {
        Some((url, token)) => (url, token),
        None => return null_link,
    };
    let mut links: Vec<Link> = Vec::new();
    let mut cursor: Option<Box<str>> = None;
    loop {
        let req = reqwest::Client::new()
            .get(&format!("{}/api/v1/list", url))
            .bearer_auth(&token);
        let req = if cursor.is_some() {
            req.query(&[("c", cursor.unwrap().as_ref())])
        } else {
            req
        };
        match req.send().await {
            Ok(x) => {
                let list_data = x.json::<Response<ListData>>().await.unwrap().data.unwrap();
                links.extend_from_slice(&list_data.links);
                cursor = list_data.cursor;
                if list_data.list_complete {
                    break;
                }
            }
            Err(e) => {
                web_sys::window()
                    .unwrap()
                    .alert_with_message(
                        format!("Failed to fetch the list. Please try again later.\n{}", e)
                            .as_str(),
                    )
                    .unwrap();
                return Box::from([]);
            }
        }
    }
    return links.into_boxed_slice();
}

pub async fn update_link(body: UpdateRequestBody) -> bool {
    let (url, token) = match get_login_info() {
        Some((url, token)) => (url, token),
        None => return false,
    };
    match reqwest::Client::new()
        .put(&format!("{}/api/v1/update", url))
        .bearer_auth(&token)
        .json(&body)
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                web_sys::window()
                    .unwrap()
                    .alert_with_message(&format!(
                        "Failed to update the link.\n\nError: {}",
                        response.json::<Response<()>>().await.unwrap().msg
                    ))
                    .unwrap();
                return false;
            }
            return true;
        }
        Err(e) => {
            web_sys::window()
                .unwrap()
                .alert_with_message(
                    format!(
                        "Failed to update the link. Please try again later.\n\nError: {}",
                        e
                    )
                    .as_str(),
                )
                .unwrap();
            return false;
        }
    };
}

pub async fn delete_link(short: &str) -> bool {
    let (url, token) = match get_login_info() {
        Some((url, token)) => (url, token),
        None => return false,
    };
    match reqwest::Client::new()
        .delete(&format!("{}/api/v1/delete", url))
        .body(format!(r#"{{"short": "{}"}}"#, short))
        .bearer_auth(&token)
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                web_sys::window()
                    .unwrap()
                    .alert_with_message(&format!(
                        "Failed to delete the link.\n\nError: {}",
                        response.json::<Response<()>>().await.unwrap().msg
                    ))
                    .unwrap();
                return false;
            }
            return true;
        }
        Err(e) => {
            web_sys::window()
                .unwrap()
                .alert_with_message(
                    format!(
                        "Failed to delete the link. Please try again later.\n\nError: {}",
                        e
                    )
                    .as_str(),
                )
                .unwrap();
            return false;
        }
    };
}

pub async fn create_link(req: CreateRequestBody) -> Box<str> {
    let null_response = Box::from("");
    let (url, token) = match get_login_info() {
        Some((url, token)) => (url, token),
        None => return null_response,
    };
    match reqwest::Client::new()
        .post(&format!("{}/api/v1/create", url))
        .bearer_auth(&token)
        .json(&req)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let response: Response<CreateData> = response.json().await.unwrap();
                return response
                    .data
                    .unwrap()
                    .short
                    .trim_start_matches(&format!(
                        "{}/",
                        url.trim_start_matches("http://")
                            .trim_start_matches("https://"),
                    ))
                    .into();
            } else {
                web_sys::window()
                    .unwrap()
                    .alert_with_message(&format!(
                        "Failed to create the link.\n\nError: {}",
                        response.json::<Response<CreateData>>().await.unwrap().msg
                    ))
                    .unwrap();
                return null_response;
            }
        }
        Err(e) => {
            web_sys::window()
                .unwrap()
                .alert_with_message(
                    format!(
                        "Failed to create the link. Please try again later.\n\nError: {}",
                        e
                    )
                    .as_str(),
                )
                .unwrap();
            return null_response;
        }
    };
}
