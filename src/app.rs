mod custom_components;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, window, MediaQueryList};
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
pub struct WindoweffectConfig {
    ifclear: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PrintlnConfig {
    msg: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component]
pub fn App() -> Html {
    html! {
        <AppUI />
    }
}
struct AppUI {
    colormode: String,
    darkmode: bool,
    window_effect: String,
}

enum Msg {
    WindoweffectChanged,
    WindoweffectChange(String),
    ColormodeChanged,
    ColormodeChange(bool),
}

impl Component for AppUI {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        // colormode 0 - check system theme
        let dark_mode = window()
            .unwrap()
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap();
        let darkmode = dark_mode.matches();
        spawn_local(async move {
            let _ = invoke(
                "tauri_println",
                to_value(&PrintlnConfig {
                    msg: format!(
                        "[INFO] Get dark mode: {}",
                        if darkmode {
                            "Dark".to_string()
                        } else {
                            "Light".to_string()
                        }
                    ),
                })
                .unwrap(),
            )
            .await;
        });
        // colormode 1 - system theme changed event binding
        let colormodechange = ctx.link().callback(Msg::ColormodeChange);
        let dark_mode_changed = Closure::wrap(Box::new(move |event: MediaQueryList| {
            let status = event.matches();
            spawn_local(async move {
                let _ = invoke(
                    "tauri_println",
                    to_value(&PrintlnConfig {
                        msg: format!(
                            "[INFO] event match status: {}",
                            if status {
                                "Dark".to_string()
                            } else {
                                "Light".to_string()
                            }
                        ),
                    })
                    .unwrap(),
                )
                .await;
            });
            colormodechange.emit(event.matches());
        }) as Box<dyn FnMut(_)>);
        dark_mode
            .add_event_listener_with_callback("change", dark_mode_changed.as_ref().unchecked_ref())
            .unwrap();
        dark_mode_changed.forget();
        // Get window effect.
        let window_effect = /*invoke("windoweffect_get", JsValue::NULL).as_string().unwrap()*/ "Mica".to_string();
        AppUI {
            colormode: "auto".to_string(),
            darkmode,
            window_effect,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ColormodeChanged => {
                if self.colormode == "auto".to_string() {
                    self.darkmode = !self.darkmode;
                    let darkmode = self.darkmode.clone();
                    spawn_local(async move {
                        let _ = invoke(
                            "colormode_change",
                            to_value(&ThemeConfig { isdark: darkmode }).unwrap(),
                        )
                        .await;
                    });
                    log_1(&JsValue::from_str(
                        format!(
                            "[INFO] Colormode changed to {}",
                            if self.darkmode {
                                "Dark".to_string()
                            } else {
                                "Light".to_string()
                            }
                        )
                        .as_str(),
                    ));
                }
            }
            Msg::ColormodeChange(isdark) => {
                if self.colormode == "auto".to_string() {
                    self.darkmode = isdark;
                    let darkmode = self.darkmode.clone();
                    spawn_local(async move {
                        let _ = invoke(
                            "colormode_change",
                            to_value(&ThemeConfig { isdark: darkmode }).unwrap(),
                        )
                        .await;
                    });
                    log_1(&JsValue::from_str(
                        format!(
                            "[INFO] Colormode changed to {}",
                            if self.darkmode {
                                "Dark".to_string()
                            } else {
                                "Light".to_string()
                            }
                        )
                        .as_str(),
                    ));
                }
            }
            Msg::WindoweffectChanged => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    let result = invoke(
                        "windoweffect_change",
                        to_value(&WindoweffectConfig { ifclear: false }).unwrap(),
                    )
                    .await
                    .as_string()
                    .unwrap();
                    link.send_message(Msg::WindoweffectChange(result))
                });
            }
            Msg::WindoweffectChange(target) => {
                self.window_effect = target;
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let colormode = if self.darkmode {
            "dark".to_string()
        } else {
            "light".to_string()
        };

        let colormode_change = ctx.link().callback(|_| Msg::ColormodeChanged);
        let windoweffect_change = ctx.link().callback(|_| Msg::WindoweffectChanged);

        let mainclass = format!("container {}", &colormode);
        let button_name = format!(
            "切换到 {}效果",
            if self.window_effect == "Mica".to_string() {
                "亚克力".to_string()
            } else {
                "云母".to_string()
            }
        );
        html! {
            <main class={&mainclass}>
                <div class="row">
                    <button onclick={colormode_change}>{"切换颜色模式"}</button>
                    <button onclick={windoweffect_change}>{button_name}</button>
                </div>
                <custom_components::TitleBar />
            </main>
        }
    }
}
