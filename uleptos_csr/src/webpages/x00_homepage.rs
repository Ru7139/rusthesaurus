use leptos::{
    html::{div, h3},
    prelude::*,
};

#[component]
pub fn Homepage() -> impl IntoView {
    div()
        .attr("data-WebPart", "Homepage")
        .style(concat!(
            "background-image: url('/public/20230917_192938_L1001333.avif');",
            "background-size: cover;",
            "background-position: center;",
            "width: 100%;",
            "height: 100vh;",
            "background-color: rgba(0, 0, 0, 0.4);",
            "background-blend-mode: darken;",
            // "background-color: rgba(137, 137, 137, 0.8);", // 半透明白色
            // "background-blend-mode: soft-light;",          // 或 overlay/soft-light
        ))
        .child(
            div()
                .style(concat!(
                    "padding: 40px;",
                    "border-radius: 20px;",
                    "background: rgba(255, 255, 255, 0.25);", // 半透明白
                    "backdrop-filter: blur(12px);",           // 毛玻璃关键
                    "-webkit-backdrop-filter: blur(12px);",   // Safari 支持
                    "border: 1px solid rgba(255, 255, 255, 0.3);", // 玻璃边缘高光
                    "box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);", // 玻璃阴影
                ))
                .child(h3().style("center").child("Welcome to Homepage")),
        )
}
