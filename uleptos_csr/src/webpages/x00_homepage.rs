use leptos::{
    html::{div, h3},
    prelude::*,
};

#[component]
pub fn Homepage() -> impl IntoView {
    div().child(h3().child("Welcome to Homepage"))
}
