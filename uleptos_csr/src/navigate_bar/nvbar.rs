use leptos::{
    html::{a, nav},
    prelude::*,
};

#[component]
pub fn NavigateBar() -> impl IntoView {
    nav()
        .attr(
            "style",
            concat!(
                "display:flex;",
                "flex-wrap:wrap;",
                "gap:1rem;",
                "align-items:center;",
                "justify-content:center;"
            ),
        )
        .child((
            a().href("/home").child("home"),
            a().href("/Home").child("Home"),
        ))
}
