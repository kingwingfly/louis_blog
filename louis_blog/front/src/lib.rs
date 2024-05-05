#[allow(unused_imports, clippy::single_component_path_imports)]
use app;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
