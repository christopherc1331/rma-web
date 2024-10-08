pub mod app;
pub mod button;
pub mod card;
pub mod input;
pub mod landing;
pub mod search_form;
pub mod artist;
pub mod nav;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
