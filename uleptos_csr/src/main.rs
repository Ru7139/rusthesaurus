mod app_with_router;
mod navigate_bar;
mod webpages;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app_with_router::app::App)

    // trunk serve --port xxxx
}
