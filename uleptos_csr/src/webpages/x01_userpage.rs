use leptos::html::{div, h3, img};
use leptos::prelude::*;

#[component]
pub fn UserMainPage() -> impl IntoView {
    div().attr("WebPart", "UserMainPage").child((
        h3().child("Here is UserMainPage"),
        div()
            .attr(
                "style",
                "display: flex; flex-direction: column; justify-content: center; align-items: center;",
            )
            .child((
                img()
                    .attr("width", "95%")
                    .alt("avif")
                    .src("public/20230917_192938_L1001333.avif"),
                img()
                    .attr("width", "95%")
                    .alt("avif")
                    .src("public/20231029_132236_L1001682.avif"),
                img()
                    .attr("width", "66%")
                    .alt("avif")
                    .src("public/20231004_135153_L1001557.avif"),
            )),
    ))
}
