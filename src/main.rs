mod create;
mod link;
mod list;
mod login;
mod utils;

use dioxus::prelude::*;

use crate::{
    create::Create,
    link::LinkItem,
    list::List,
    login::{check_local_login_info, LoginForm},
};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let checked = use_resource(|| check_local_login_info());
    rsx! {
        link { rel: "icon", href: FAVICON, r#type: "image/svg+xml" }
        document::Stylesheet { href: TAILWIND_CSS }
        if let Some(true) = checked() {
            Router::<Route> {}
        } else {
            LoginForm {}
        }
    }
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)]
    #[layout(SideBar)]
    #[route("/")]
    Home,
    #[route("/list")]
    List,
    #[route("/link/:link")]
    LinkItem { link: String },
    #[route("/create")]
    Create,
    #[route("/:..s")]
    NotFound { s: Vec<String> },
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        header { class: "flex justify-between items-center border border-gray-300 bg-white sticky top-0",
            span { class: "pe-px" }
            h1 { class: "text-xl font-semibold text-gray-800", "linkrusk" }
            div { class: "flex items-stretch h-7 hover:bg-gray-200",
                span { class: "w-px bg-gray-300 mr-2" }
                Link {
                    to: "/",
                    onclick: |_| {
                        let window = web_sys::window().unwrap();
                        window.local_storage().unwrap().unwrap().clear().unwrap();
                        window.location().reload().unwrap();
                    },
                    "Logout"
                }
                span { class: "w-px ml-2" }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn SideBar() -> Element {
    rsx! {
        aside { class: "w-20 bg-white border-r border-b border-gray-300
                        fixed top-7.1 left-0 h-full flex flex-col",
            nav {
                ul {
                    li { class: "flex items-stretch h-7 w-20 hover:bg-gray-200 border-b border-gray-300",
                        Link { class: "px-4.5", to: Route::Home, "Home" }
                    }
                    li { class: "flex items-stretch h-7 w-20 hover:bg-gray-200 border-b border-gray-300",
                        Link { class: "px-4.5", to: Route::List, "List" }
                    }
                    li { class: "flex items-stretch h-7 w-20 hover:bg-gray-200 border-b border-gray-300",
                        Link { class: "px-4.5", to: Route::Create, "Create" }
                    }
                }
            }
        }
        div { class: "pl-20", Outlet::<Route> {} }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            h1 { class: "text-3xl px-5", "Welcome to linkrusk!" }
        }
    }
}

#[component]
pub fn NotFound(s: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { class: "text-3xl px-5", "Page not found!" }
        }
    }
}
