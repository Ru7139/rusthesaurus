use leptos::html::{div, h3};
use leptos::prelude::*;

#[component]
pub fn UserMainPage() -> impl IntoView {
    div()
        .attr("id", "UserMainPage")
        .child(h3().child("Here is UserMainPage"))
}
