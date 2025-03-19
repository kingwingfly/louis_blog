use leptos::prelude::*;
use leptos_router::components::{Form, Outlet};

#[component]
pub fn Auth() -> impl IntoView {
    view! {
        <h1>"Auth"</h1>
        <Outlet/>
    }
}

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <h2>"Login"</h2>
    }
}

#[component]
pub fn Register() -> impl IntoView {
    view! {
        <h2>"Register"</h2>
        <Form method="POST" action="/api/register">
            <label for="username">"Username"</label>
            <input type="text" name="username"/>
            <label for="email">"Email"</label>
            <input type="email" name="email"/>
            <label for="password">"Password"</label>
            <input type="password" name="password"/>
            <input type="submit" value="Register"/>
        </Form>
    }
}
