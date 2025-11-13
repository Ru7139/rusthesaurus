use leptos::{
    html::{a, div, nav},
    prelude::*,
};

#[component]
pub fn App() -> impl IntoView {
    div().child(
        nav()
            .attr("class", "hidden md:flex items-center gap-6 text-sm")
            .child((
                a().href("#features")
                    .class("hover:underline")
                    .child("Features"),
                a().href("#pricing")
                    .class("hover:underline")
                    .child("Pricing"),
                a().href("#about").class("hover:underline").child("About"),
                a().href("#contact")
                    .class("hover:underline")
                    .child("Contact"),
            )),
    )
}
