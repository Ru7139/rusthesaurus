use leptos::html::{ElementChild, div, main};
use leptos::prelude::*;
use leptos_meta::{MetaProps, StylesheetProps, TitleProps};
use leptos_router::{
    StaticSegment,
    components::{Route, RouteProps, RouterProps, RoutesProps},
};

#[rustfmt::skip]
#[component]
pub fn App() -> impl IntoView {
    div().child((
        leptos_meta::Stylesheet(StylesheetProps::builder().id("web-css").href("/web_style.css").build()),
        leptos_meta::Title(TitleProps::builder().text("uleptos_csr").build()),
        leptos_meta::Meta(MetaProps::builder().name("color-scheme").content("dark").build()),
        leptos_router::components::Router(RouterProps::builder().children(ToChildren::to_children(|| {(
            crate::navigate_bar::nvbar::NavigateBar(),
            main().child(leptos_router::components::Routes(RoutesProps::builder()
                .fallback(|| "Page Not Found")
                .children(ToChildren::to_children(||{(
                    Route(RouteProps::builder()
                        .path(StaticSegment("Home"))
                        .view(crate::webpages::x00_homepage::Homepage).build()
                    ),
                    Route(RouteProps::builder()
                        .path(StaticSegment("User"))
                        .view(crate::webpages::x01_userpage::UserMainPage).build()
                    ),
                )})).build()))
            )})).build())
    ))
}
