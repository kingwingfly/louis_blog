#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[allow(unused)]
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
