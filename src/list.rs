use dioxus::prelude::*;

use crate::utils::{self, Link};

#[component]
pub fn List() -> Element {
    #[allow(clippy::redundant_closure)]
    let links = use_resource(|| utils::fetch_links());
    rsx! {
        match links() {
            Some(links) => render_links(links.as_ref()),
            None => rsx! {
                div { class: "mb-2 text-2xl", "Loading..." }
            },
        }
    }
}

fn render_links(links: &[Link]) -> Element {
    rsx! {
        for link in links.iter() {
            Link { to: format!("/link/{}", link.short.key),
                LinkComponent { link: link.clone() }
            }
        }
        if links.is_empty() {
            p { class: "text-gray-500", "No links found." }
        }
    }
}

#[component]
fn LinkComponent(link: Link) -> Element {
    let key = link.short.key;
    let url = link.url.unwrap_or(Box::from("Failed to display"));
    rsx! {
        div { class: "border-r border-b border-gray-300 p-4 hover:bg-gray-100",
            p { class: "mb-2", "Key: {key}" }
            p { class: "mb-2 break-all", "To URL:
                {url}" }
        }
    }
}
