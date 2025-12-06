use leptos::html::{ElementChild, div};
use leptos::prelude::*;
use leptos_meta::{MetaProps, StylesheetProps, TitleProps};
use leptos_router::components::{Router, RouterProps};

#[rustfmt::skip]
#[component]
pub fn App() -> impl IntoView {

    div().child((
        leptos_meta::Stylesheet(StylesheetProps::builder().id("web-css").href("/web_style.css").build()),
        leptos_meta::Title(TitleProps::builder().text("uleptos_csr").build()),
        leptos_meta::Meta(MetaProps::builder().name("color-scheme").content("dark").build()),

        Router(RouterProps::builder().children(ToChildren::to_children(|| {(
            crate::navigate_bar::nvbar::NavigateBar(),
            crate::app_with_router::catsroutes::CatsRoutes(),
            )})).build())
    ))
}
