use leptos::{
    html::{div, main},
    prelude::*,
};
use leptos_router::{
    StaticSegment,
    components::{Route, RouteProps, RoutesProps},
};

macro_rules! non_ssr_route_build {
    ($webpath:expr, $webpage:path) => {
        Route(
            RouteProps::builder()
                .path(StaticSegment($webpath))
                .view($webpage)
                .build(),
        )
    };
}

#[rustfmt::skip]
#[component]
pub fn CatsRoutes() -> impl IntoView {
    use crate::webpages;

    div().attr("WebPart", "CatsRoutes")
        .child(
        main().child(leptos_router::components::Routes(
            RoutesProps::builder()
                .fallback(|| "Page Not Found")
                .children(ToChildren::to_children(|| {
                    (
                        non_ssr_route_build!("Home", webpages::x00_homepage::Homepage),
                        non_ssr_route_build!("User", webpages::x01_userpage::UserMainPage),
                        non_ssr_route_build!("LeptosDev",webpages::x99_example::BasicComponts),
                    )
                }))
                .build(),
        )),
    )
}
