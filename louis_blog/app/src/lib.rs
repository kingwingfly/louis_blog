mod auth;
mod home;
mod user;

use auth::{Auth, Login, Register};
use home::HomePage;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use user::{NoUser, UserProfile, Users};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute path=path!("/users") view=Users>
                        <Route path=path!("/me") view=UserProfile/>
                        <Route path=path!("/") view=NoUser/>
                    </ParentRoute>
                    <ParentRoute path=path!("/auth") view=Auth>
                        <Route path=path!("/login") view=Login/>
                        <Route path=path!("/register") view=Register/>
                    </ParentRoute>
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
