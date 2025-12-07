use leptos::{
    html::{a, div, nav},
    prelude::*,
};

#[component]
pub fn NavigateBar() -> impl IntoView {
    div().attr("data-WebPart", "NavigateBar").child(
        nav()
            .style(concat!(
                "width: 100%;",
                "position: fixed;",
                "z-index: 99;",
                "padding: 0.8% 0 1.5%;",
                "text-align: center;",
                "font-family: the-seasons, sans-serif;",
                "color: white;",
                "font-size: 1em;",
                "display:flex;",
                "flex-wrap:wrap;",
                "gap:1rem;",
                "justify-content:center;", // 让内部元素置中
                "align-items:center;",     // 让垂直方向也对齐得更好
            ))
            .child((
                a().href("/Home").target("_self").child("Home"), // 在当前页打开，_blank在新的标签页打开
                a().href("/User").child("User"),
                "|",
                a().href("/LeptosDev").child("Leptos-Dev-Book"),
            )),
    )
}
