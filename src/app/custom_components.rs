use std::f32::consts::E;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct PrintlnConfig {
    msg: String,
}

pub struct TitleBar {
    title: String,
}

pub enum TitlebarMsg {
    Minimize,
    Close,
}

impl Component for TitleBar {
    type Properties = ();
    type Message = TitlebarMsg;
    fn create(ctx: &Context<Self>) -> Self {
        TitleBar {
            title: "TimeUp".to_string(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TitlebarMsg::Minimize => {
                spawn_local(async {
                    let _ = invoke("window_minimize", JsValue::null()).await;
                });
            }
            TitlebarMsg::Close => {
                spawn_local(async {
                    let _ = invoke("window_close", JsValue::null()).await;
                });
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let minimize = ctx.link().callback(|_| TitlebarMsg::Minimize);
        let close = ctx.link().callback(|_| TitlebarMsg::Close);
        html! {
            <div id="titlebar" data-tauri-drag-region="true" class="titlebar">
                <div id="titlebar-title" class="titlebar-title">
                    <label>{ &*self.title }</label>
                </div>
                <div id="titlebar-minimize" class="titlebar-button" onclick={minimize}>{'-'}</div>
                <div id="titlebar-close" class="titlebar-closebutton" onclick={close}>{'\u{00d7}'}</div>
            </div>
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum StaskStatus {
    Wait,
    Completed,
    NoNeed,
}

struct TaskItem {
    completed: Option<StaskStatus>,
    description: String,
    outdate: Option<bool>,
    show_menu: bool,
}

enum ItemMsg {
    Finish,
}

impl Component for TaskItem {
    type Properties = ();
    type Message = ItemMsg;
    fn create(ctx: &Context<Self>) -> Self {
        TaskItem {
            completed: Some(StaskStatus::Wait),
            description: "".to_string(),
            outdate: None,
            show_menu: false,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ItemMsg::Finish => {
                if self.completed == Some(StaskStatus::Completed) {
                    self.completed = Some(StaskStatus::Wait);
                } else {
                    self.completed = Some(StaskStatus::Completed);
                };
                // spawn_local(async move {
                //     let _ = invoke(
                //         "tauri_println",
                //         to_value(&PrintlnConfig {
                //             msg: format!("[INFO] Taskitem clicked"),
                //         })
                //         .unwrap(),
                //     )
                //     .await;
                // });
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let checkbox = "〇".to_string();
        let itemclass = format!("taskitem {:?}", self.completed.clone().unwrap());
        html! {
            <div class={&itemclass} onclick={ctx.link().callback(|_| ItemMsg::Finish)}>
                <div class="taskitem-content">
                    <label>{&checkbox}</label>
                    <input value="空任务" disabled=true />
                </div>
                {if self.show_menu {
                    html! {
                        <div class="taskitem-menu">
                            {"This is How it shows."}
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }
}

pub struct TaskList {
    tasks: Vec<TaskItem>,
}

pub enum TaskMsg {}

impl Component for TaskList {
    type Properties = ();
    type Message = TaskMsg;
    fn create(ctx: &Context<Self>) -> Self {
        TaskList { tasks: Vec::new() }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tasklist">
                <div class="tasks">
                <TaskItem />
                <TaskItem />
                <TaskItem />
                <TaskItem />
                <TaskItem />
                <TaskItem />
                <TaskItem />
                </div>
            </div>
        }
    }
}

pub struct TabBar {
    page_number: i8,
}

pub enum TabBarMsg {
    SwitchPage(i8),
}

impl Component for TabBar {
    type Properties = ();
    type Message = TabBarMsg;
    fn create(ctx: &Context<Self>) -> Self {
        TabBar { page_number: 0 }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabBarMsg::SwitchPage(page) => {
                if self.page_number == page {
                    self.page_number = 0;
                    
                }
                else {
                    self.page_number = page;

                }
            }
            
        };
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let style1 = format!("color: {}", if self.page_number == 1 {"#396cd8"} else {"unset"});
        let style2 = format!("color: {}", if self.page_number == 2 {"#396cd8"} else {"unset"});
        html! {
            <div class="tabbar">
                <div class="tabbar-button">
                    {'a'}
                </div>
                <div class="space"/>
                <div class="tabbar-button" onclick={ctx.link().callback(|_| TabBarMsg::SwitchPage(1))} style={style1}>
                    {if self.page_number == 1 {'T'} else {'t'}}
                </div>
                <div class="tabbar-button" onclick={ctx.link().callback(|_| TabBarMsg::SwitchPage(2))} style={style2}>
                    {if self.page_number == 2 {'S'} else {'s'}}
                </div>
            </div>
        }
    }
}
