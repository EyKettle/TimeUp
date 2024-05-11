use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(TitleBar)]
pub fn titlebar() -> Html {
    let title = use_state(|| "TimeUp".to_string());

    let minimize = {
        Callback::from(move |_| {
            invoke("window_minimize", JsValue::null());
        })
    };
    let close = {
        Callback::from(move |_| {
            invoke("window_close", JsValue::null());
        })
    };

    html! {
        <div id="titlebar" data-tauri-drag-region="true" class="titlebar">
            <div id="titlebar-title" class="titlebar-title">
                <label>{ &*title }</label>
            </div>
            <div id="titlebar-minimize" class="titlebar-button" onclick={minimize}>{'-'}</div>
            <div id="titlebar-close" class="titlebar-closebutton" onclick={close}>{'\u{00d7}'}</div>
        </div>
    }
}
