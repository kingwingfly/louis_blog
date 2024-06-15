use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    #[allow(unused_imports)]
    use app::*;

    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
