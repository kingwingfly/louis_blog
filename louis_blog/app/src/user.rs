use gloo_net::http::Request;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use serde::{Deserialize, Serialize};

#[component]
pub fn Users() -> impl IntoView {
    view! {
        <h1>"User Profile"</h1>
        <Outlet/>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WhoAmI {
    id: i32,
    name: String,
}

#[island]
pub fn UserProfile() -> impl IntoView {
    let whoami = LocalResource::new(move || async {
        let resp = Request::get("/api/whoami")
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let body = resp.text().await.map_err(|e| e.to_string())?;
        match serde_json::from_str::<WhoAmI>(&body) {
            Ok(whoami) => Ok(whoami),
            Err(_) => Err(body.to_string()),
        }
    });
    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> } >
            {move || match whoami.get().as_deref() {
                Some(Ok(whoami)) => Some(view! { <p>{format!("Hello, {}!", whoami.name)}</p> }),
                Some(Err(e)) => Some(view! { <p>{e.to_string()}</p> }),
                None => None,
            }}
        </Transition>
    }
}

#[component]
pub fn NoUser() -> impl IntoView {}
