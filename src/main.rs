use leptos::prelude::*;
use leptos_router::components::Router;

use mpkr25::MPKR;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! {
            <Router>
                <MPKR />
            </Router>
            <noscript>"This page contains webassembly and javascript content, please enable javascript in your browser."</noscript>
        }
    })
}
