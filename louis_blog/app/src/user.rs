use crate::auth::{DeleteAccount, Login, Logout};
use gloo_net::http::Request;
use leptos::{either::Either, prelude::*};
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

#[component]
pub fn UserProfile() -> impl IntoView {
    view! {
        <div>
            <Me/>
        </div>
    }
}

#[island]
pub fn Me() -> impl IntoView {
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
                Some(Ok(whoami)) => Some(Either::Left(view! {
                    <p>{format!("Hello, {}!", whoami.name)}</p>
                    <Logout/>
                    <DeleteAccount/>
                })),
                Some(Err(e)) => Some(Either::Right(view! {
                    <p>{e.to_string()}</p>
                    <Login/>
                })),
                None => None,
            }}
        </Transition>
    }
}

#[component]
pub fn NoUser() -> impl IntoView {}
