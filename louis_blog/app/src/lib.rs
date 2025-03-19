mod user;

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
                        <Route path=path!("me") view=UserProfile/>
                        <Route path=path!("") view=NoUser/>
                    </ParentRoute>
                    <Route path=path!("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Counter/>
    }
}

#[island]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
