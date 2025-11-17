use leptos::{
    html::{div, h3},
    prelude::*,
};

#[component]
pub fn Homepage() -> impl IntoView {
    div()
        .attr("WebPart", "Homepage")
        .child(h3().child("Welcome to Homepage"))
}
