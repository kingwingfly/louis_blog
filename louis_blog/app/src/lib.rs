use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[island]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hi from your Leptos WASM!"</h1>
    }
}

#[island]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div>
            <Stylesheet id="leptos" href="/pkg/louis_blog.css"/>
            <Title text="Louis' blogs" />
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Meta name="description" content="Louis' blogs"/>
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=HomePage/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
