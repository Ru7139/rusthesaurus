use leptos::{
    html::{a, nav},
    prelude::*,
};

#[component]
pub fn NavigateBar() -> impl IntoView {
    nav()
        .attr("id", "NavigateBar")
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
            a().href("/Home").child("Home"),
            a().href("/User").child("User"),
        ))
}
