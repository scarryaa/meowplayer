use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::divider::Divider;
use crate::app::side_panel::SidePanel;
use crate::app::title_bar::TitleBar;
use crate::app::top_panel::TopPanel;

mod divider;
mod side_panel;
mod title_bar;
mod top_panel;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <TitleBar />
            <Divider />
            <TopPanel />
            <SidePanel />
        </>
    }
}
