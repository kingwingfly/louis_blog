use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Louis' blog!"</h1>
        <A href="/auth/login">Login</A>
    }
}
