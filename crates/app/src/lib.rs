pub mod components;
pub mod config;
pub mod data;
pub mod pages;

use components::{Footer, Header};
use leptos::*;
use leptos_router::*;
use pages::{Contact, Home, LinuxJourney, Projects};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="min-h-screen flex flex-col">
                <Header />
                <main class="flex-1 flex flex-col items-center px-6 md:px-20 lg:px-40 py-8">
                    <Routes>
                        <Route path="/" view=Home />
                        <Route path="/projects" view=Projects />
                        <Route path="/linux-journey" view=LinuxJourney />
                        <Route path="/contact" view=Contact />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}

pub fn mount() {
    if let Some(window) = web_sys::window() {
        if let Ok(search) = window.location().search() {
            if search.contains("redirect=") {
                let path = search
                    .trim_start_matches('?')
                    .split('&')
                    .find(|p| p.starts_with("redirect="))
                    .and_then(|p| p.strip_prefix("redirect="))
                    .unwrap_or("/");
                let decoded = js_sys::decode_uri_component(path)
                    .ok()
                    .and_then(|s| s.as_string())
                    .unwrap_or_else(|| "/".to_string());
                let _ = window.history().ok().and_then(|h| {
                    h.replace_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&decoded),
                    )
                    .ok()
                });
            }
        }
    }
    mount_to_body(App);
}
