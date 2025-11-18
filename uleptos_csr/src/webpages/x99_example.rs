use leptos::{
    ev,
    html::{br, button, div, h3, hr, li, p, progress, ul},
    prelude::*,
};

#[component]
pub fn BasicComponts() -> impl IntoView {
    let (click_collector, click_collector_set) = signal(0u32);
    let (vec_clicks_c, vec_clicks_c_set) = signal(Vec::<u32>::new()); // 可以使用with_capacity(100_000)的方式减少延迟

    div().attr("data-WebPart", "BasicComponent").child((
        div()
            .attr("data-LeptosDev", "click and progress bar")
            .child((
                h3().child("Basic Components from Leptos-Dev-Book"),
                p().child("click_collector: ")
                    .child(move || click_collector.get()),
                progress()
                    .max(50000u32)
                    .value(move || click_collector.get()),
                br(),
                button()
                    .on(ev::click, move |_click| {
                        *click_collector_set.write() += 10000;
                        vec_clicks_c_set
                            .update(|x| *x = (0..click_collector.get()).collect::<Vec<u32>>());
                    })
                    .class(("red", move || click_collector.get() % 3 == 0))
                    .child("+10000"),
                br(),
                button()
                    .on(ev::click, move |_click| {
                        vec_clicks_c_set.update(|x| {
                            // 只追加新项，不重建整个 Vec
                            let start = x.len() as u32;
                            x.extend(start..(start + 10000));
                        });
                    })
                    .child("extend +10000"),
            )),
        hr(),
        div().attr("data-WebPart", "vec iteration").child((
            button()
                .on(ev::click, move |_click| {
                    vec_clicks_c_set.update(|x| {
                        x.iter_mut().enumerate().for_each(|(index, num)| {
                            if index % 3 == 0 {
                                *num += 1
                            }
                        })
                    });
                })
                .child("+1 to vec"),
            br(),
            ul().child(move || {
                vec_clicks_c
                    .get()
                    .iter()
                    .copied()
                    .map(|x| li().child(x))
                    .collect_view()
            }), // 180 ~ 240 ~ 320
            hr(),
            // ul().child(ForEnumerate(
            //     ForEnumerateProps::builder()
            //         .each(move || vec_clicks_c.get())
            //         .key(|counter| *counter)
            //         .children(move |idx, counter| {
            //             li().child(move || idx.get()).child(":").child(counter)
            //         })
            //         .build(),
            // )), // 完全没有变快的感觉，反而是全局更新的效果更好 // 或许在分别只更改单个元素的场景下才应该使用
        )),
    ))
}
