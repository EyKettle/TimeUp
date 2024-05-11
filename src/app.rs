mod custom_components;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{window, MediaQueryList};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub version: String,
    pub colormode: String,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeConfig {
    isdark: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PrintlnConfig {
    msg: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

struct App {

}

impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let configs = use_state(|| Config {
            version: "0.0.0".to_string(),
            colormode: "light".to_string(),
        });
        use_effect({
            let configs = configs.clone();
            move || {
                let config: Config = from_value(invoke("config_load", JsValue::null())).unwrap();
                configs.set(config);
                || {}
            }
        });
    
        let window = window().unwrap();
        let isdark = if configs.colormode == "auto".to_string() {
            invoke(
                "tauri_println",
                to_value(&PrintlnConfig {
                    msg: format!("Initialize colormode by [auto].").to_string(),
                })
                .unwrap(),
            );
            let result = window
                .match_media("(prefers-color-scheme: dark)")
                .unwrap()
                .unwrap()
                .matches();
            result
        } else {
            invoke(
                "tauri_println",
                to_value(&PrintlnConfig {
                    msg: format!(
                        "Initialize colormode by [config]. colormode = {}",
                        configs.colormode
                    )
                    .to_string(),
                })
                .unwrap(),
            );
            configs.colormode == "dark".to_string()
        };
    
        let color_mode: UseStateHandle<String> = use_state(|| {
            invoke(
                "tauri_println",
                to_value(&PrintlnConfig {
                    msg: format!("Initialize container colormode. isdark: {isdark}").to_string(),
                })
                .unwrap(),
            );
            invoke(
                "colormode_change",
                to_value(&ThemeConfig { isdark: isdark }).unwrap(),
            );
            if isdark {
                "container dark".to_string()
            } else {
                "container".to_string()
            }
        });
    
        let colormode_change = {
            let color_mode = color_mode.clone();
            move |_| {
                let mut args = ThemeConfig { isdark: isdark };
                if color_mode.as_ref() == "container".to_string() {
                    color_mode.set("container dark".to_string());
                    args.isdark = true;
                } else {
                    color_mode.set("container".to_string());
                    args.isdark = false;
                }
                invoke("colormode_change", to_value(&args).unwrap());
            }
        };
    
        let closure = Closure::wrap(Box::new({
            let color_mode = color_mode.clone();
            move |event: MediaQueryList| {
                if event.matches() {
                    let args = ThemeConfig { isdark: true };
                    color_mode.set("container dark".to_string());
                    invoke("colormode_change", to_value(&args).unwrap());
                } else {
                    let args = ThemeConfig { isdark: false };
                    color_mode.set("container".to_string());
                    invoke("colormode_change", to_value(&args).unwrap());
                }
            }
        }) as Box<dyn FnMut(_)>);
        window
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap()
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .expect("Failed to add event listener");
        closure.forget();
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class={&*color_mode}>
                <div class="row">
                    <a href="https://tauri.app" target="_blank">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://yew.rs" target="_blank">
                        <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                    </a>
                    <button onclick={colormode_change}>{"切换颜色模式"}</button>
                </div>
                <custom_components::TitleBar />
            </main>
        }
    }
}