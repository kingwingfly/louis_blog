use leptos::prelude::*;
use leptos_router::components::{Form, Outlet, A};

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
        <Form method="POST" action="/api/login" replace=true>
            <label for="email">"Email"</label>
            <input type="email" name="email"/>
            <label for="password">"Password"</label>
            <input type="password" name="password"/>
            <input type="submit" value="Login"/>
        </Form>
        <A href="/auth/register">Register</A>
    }
}

#[component]
pub fn Register() -> impl IntoView {
    view! {
        <h2>"Register"</h2>
        <Form method="POST" action="/api/register" replace=true>
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

#[component]
pub fn Logout() -> impl IntoView {
    view! {
        <Form method="GET" action="/api/logout" replace=true>
            <input type="submit" value="Logout"/>
        </Form>
    }
}

#[component]
pub fn DeleteAccount() -> impl IntoView {
    view! {
        <Form method="POST" action="/api/delete_account" replace=true>
            <input type="submit" value="Delete Account"/>
        </Form>
    }
}
