use dioxus::prelude::*;
use dioxus_router::use_router;

use crate::SearchState;
// use dioxus_router::use_history;
// use dioxus_state::use_state;

// pub fn SearchBar(cx: Scope) -> Element {
//     // let (search_query, set_search_query) = use_state(String::new);
//     let search_query = use_state(cx, String::new);
//     // let router = use_router(cx);
//     // let route = use_route(cx);
//     // let history = use_history();

//     let on_input = use_callback!(cx, move |_| async move {
//         log::info!("Hello 2");
//         // let target = event.target().unwrap();
//         // let input_element = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
//         // let value = input_element.value();
//         // set_search_query(value);
//     });

//     cx.render(rsx! {
//         div {
//             class: "w-full",
//             svg {
//                 class: "w-6 h-6",
//                 fill: "none",
//                 view_box: "0 0 24 24",
//                 stroke_width: "1.5",
//                 stroke: "white",
//                 path {
//                     stroke_linecap: "round",
//                     stroke_linejoin: "round",
//                     d: "M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z",
//                 }
//             }
//             input {
//                 class: "border bg-transparent text-slate-100 p-4 w-full max-w-3xl",
//                 r#type: "text",
//                 placeholder: "Search",
//                 value: "{search_query}",
//                 oninput: on_input,
//                 div {
//                     class: "w-4 h-4 bg-red-500 left-0 top-0",
//                 }
//             }
//         }
//     })
// }

pub fn SearchPage(cx: Scope) -> Element {
    let search_state = use_shared_state::<SearchState>(cx).unwrap();

    let r_search_state = search_state.read();

    if r_search_state.is_searching {
        cx.render(rsx! {
            div {
                onclick: move |_| {
                    let mut w_search_state = search_state.write();
                    w_search_state.is_searching = false;
                },
                class: "absolute top-0 left-0 w-screen h-screen backdrop-blur content-center flex flex-col",
                input {
                    class: "border border-slate-700 rounded bg-slate-900 text-slate-100 p-4 w-full max-w-3xl mx-auto mt-32",
                    r#type: "text",
                    placeholder: "Search",
                    // value: "{search_query}",
                    // oninput: on_input,
                    // div {
                    //     class: "w-4 h-4 bg-red-500 left-0 top-0",
                    // }
                }
            }
        })
    } else {
        None
    }
}

pub fn SearchButton(cx: Scope) -> Element {
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    cx.render(rsx! {
        button {
            class: "rounded-full bg-transparent text-slate-100 hover:text-slate-900 hover:bg-slate-100 p-3",
            onclick: move |_| {
                let mut w_search_state = search_state.write();
                w_search_state.is_searching = true;
            },
            svg {
                class: "w-6 h-6",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z",
                }
            }
        }
    })
}
