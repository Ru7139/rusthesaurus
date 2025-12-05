use leptos::{
    html::{a, div, nav},
    prelude::*,
};

#[component]
pub fn NavigateBar() -> impl IntoView {
    div().attr("data-WebPart", "NavigateBar").child(
        nav()
            .style(concat!(
                "display:flex;",
                "flex-wrap:wrap;",
                "gap:1rem;",
                "align-items:center;",
                "justify-content:center;",
                "padding:1rem 0;",
            ))
            .child((
                a().href("/Home").target("_self").child("Home"), // 在当前页打开，_blank在新的标签页打开
                a().href("/User").child("User"),
                a().href("/LeptosDev").child("Leptos-Dev-Book"),
            )),
    )
}
