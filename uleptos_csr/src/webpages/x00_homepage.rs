use leptos::{
    html::{canvas, div, h3},
    prelude::*,
};

#[component]
pub fn Homepage() -> impl IntoView {
    div().attr("WebPart", "Homepage").child((
        h3().child("Welcome to Homepage"),
        canvas().width(500).height(500).child("Canvas"),
    ))
}
