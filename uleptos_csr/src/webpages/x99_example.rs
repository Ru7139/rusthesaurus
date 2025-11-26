use leptos::{
    ev,
    html::{br, button, div, h3, hr, li, p, progress, ul},
    prelude::*,
};

use uuid::Uuid;

#[component]
pub fn BasicComponts() -> impl IntoView {
    let (click_collector, click_collector_set) = signal(0u32);
    // 可以使用with_capacity(100_000)的方式减少延迟，但不明显
    let (uuid_click, uuid_click_set) = signal(Vec::<Uuid>::new());

    div().attr("data-WebPart", "BasicComponent").child((
        div()
            .attr("data-LeptosDev", "click and progress bar")
            .child((
                h3().child("Basic Components from Leptos-Dev-Book"),
                p().child("click_collector: ")
                    .child(move || click_collector.get()),
                progress().max(5000u32).value(move || click_collector.get()),
                br(),
                button()
                    .on(ev::click, move |_click| {
                        *click_collector_set.write() += 1000;
                        uuid_click_set.update(|x| {
                            *x = (0..click_collector.get())
                                .into_iter()
                                .map(|_| Uuid::new_v4())
                                .collect::<Vec<Uuid>>()
                        });
                    })
                    .class(("red", move || click_collector.get() % 3 == 0))
                    .child("+1000"),
                br(),
                button()
                    .on(ev::click, move |_click| {
                        *click_collector_set.write() += 1000;
                        uuid_click_set.update(|x| {
                            // 只追加新项，不重建整个 Vec
                            x.extend((0..1000).into_iter().map(|_| Uuid::new_v4()));
                        });
                    })
                    .child("extend +1000"),
            )),
        hr(),
        div().attr("data-WebPart", "vec iteration").child((
            button()
                .on(ev::click, move |_click| {
                    uuid_click_set.update(|x| {
                        x.iter_mut().enumerate().for_each(|(idx, id)| {
                            if idx % 3 == 0 {
                                *id = Uuid::new_v4();
                            }
                        });
                    });
                })
                .child("+1 to vec"),
            br(),
            ul().child(move || {
                uuid_click
                    .get()
                    .iter()
                    .copied()
                    .map(|x| li().child(x.to_string()))
                    .collect_view()
            }), // 180 ~ 240 ~ 320
            hr(),
            // ul().child(ForEnumerate(
            //     ForEnumerateProps::builder()
            //         .each(move || uuid_click.get())
            //         .key(|counter| *counter)
            //         .children(move |idx, uuid_value| {
            //             li().child(move || idx.get())
            //                 .child(": ")
            //                 .child(uuid_value.to_string())
            //         })
            //         .build(),
            // )), // 完全没有变快的感觉，反而是全局更新的效果更好 // 或许在分别只更改单个元素的场景下才应该使用
        )),
    ))
}
