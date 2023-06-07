use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[function_component(TitleBar)]
pub fn title_bar() -> Html {
    html! {
        <>
            <div class="resize-margin"/>
            <div class="titlebar" data-tauri-drag-region="true"/>
        </>
    }
}
