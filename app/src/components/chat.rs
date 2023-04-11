use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use reqwest_wasm::{header::{CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN}, Body};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn Chat(cx: Scope) -> Element {
    let chat_state = use_state::<ChatState>(cx, || ChatState::default());

    cx.render(rsx! {
        div {
            class: "absolute inset-x-0 bottom-0 flex flex-col-reverse max-h-72 px-3 pb-2 bg-slate-800 rounded-xl",
            ChatBar {
                chat_state: chat_state.clone()
            }
            ChatResults {
                chat_state:chat_state.clone()
            }
        }
    })
}

#[inline_props]
pub fn ChatBar(cx: Scope, chat_state: UseState<ChatState>) -> Element {
    let query = &chat_state.get().query.clone();

    // Move the focus to the chat bar.
    // autofocus property on input is having issues: https://github.com/DioxusLabs/dioxus/issues/725
    use_effect(&cx, (), |_| async move {
        gloo_timers::future::TimeoutFuture::new(50).await;
        let document = gloo_utils::document();
        if let Some(element) = document.get_element_by_id("chat-bar") {
            element.unchecked_into::<HtmlElement>().focus().ok();
        }
    });


    use_future!(cx, |(chat_state,)| {
        async move {
            if chat_state.busy && chat_state.query.len() > 0 {
                let payload = serde_json::json!({
                    "message": "Hello, my name is Nick. What is your name?"
                });
                let res = reqwest::Client::new()
                    .post("http://127.0.0.1:5000/chat")
                    .header(CONTENT_TYPE, "application/json")
                    .json(&payload)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                log::info!("response: {:#?}", res);
                chat_state.modify(|cs| ChatState { busy: false, query:"".to_string(), results: cs.results.clone()});
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "relative flex flex-row mt-2 rounded-xl gap-x-4 self-stretch lg:gap-x-6 shadow-sm",
            input {
                class: "w-full h-12 bg-slate-700 rounded-xl border-0 px-4 py-1.5 text-gray-100 shadow-sm ring-0.5 placeholder:text-gray-400 focus:ring-0 focus:outline-0 sm:text-lg sm:leading-6",
                id: "chat-bar",
                r#type: "text",
                placeholder: "Send a message to ClockworkGPT...",
                value: "{query}",
                oninput: move |e| {
                    let query_str = e.value.clone().as_str().to_string();
                    chat_state.modify(|cs| ChatState {query: query_str, results: cs.results.clone(), ..*cs});
                },
                onclick: move |e| e.stop_propagation(),
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        chat_state.modify(|cs| ChatState { query: cs.query.clone(), results: cs.results.clone(), busy: true});
                    } 
                }
            }
            button {
                onclick: move |_| {
                    chat_state.modify(|cs| ChatState { query: cs.query.clone(), results: cs.results.clone(), busy: true});

                },
                class: "absolute inset-y-0 right-4 px-2 my-1 text-slate-100 hover:bg-slate-600 active:bg-slate-400 active:text-slate-900 active:ring-0 active:focus-0 transition text-sm font-medium rounded",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "1.5",
                    stroke: "currentColor",
                    class: "w-5 h-5",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5"  
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn ChatResults(cx: Scope, chat_state: UseState<ChatState>) -> Element {
    let query = &chat_state.get().query;

    // Search for search results.
    let results = use_future(&cx, query, |_| {
        let _query = query.clone();
        async move {
            let x: Vec<String> = Vec::new();
            x
        }
    });

    if let Some(chat_results) = results.value() {
        cx.render(rsx! {
            div {
                class: "flex flex-col w-full mt-2 space-y-4 mx-auto px-4 overflow-y-auto",
                for chat_result in chat_results.iter() {
                    rsx! {
                        ChatResultRow {
                            chat_result: chat_result.to_string(),
                        }
                    }
                }
            }
        })
    } else {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    message: String
}

#[derive(PartialEq, Clone, Props)]
pub struct ChatResultRowProps {
    pub chat_result: String 
}

pub fn ChatResultRow(cx: Scope<ChatResultRowProps>) -> Element {
    cx.render(rsx! {
        p {
            class: "text-md text-slate-400",
            "{cx.props.chat_result}"
        }
    })
}

#[derive(Debug, Default)]
pub struct ChatState {
    pub busy: bool,
    pub query: String,
    pub results: Vec<String>,
}