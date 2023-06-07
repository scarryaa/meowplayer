use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[function_component(TopPanel)]
pub fn top_panel() -> Html {
    html! {
        <div class="top-panel">
            <div class="top-panel__controls">
                <div class="top-panel__controls__main">
                    <div class="top-panel__controls__main__container flex-row">
                        <i class="material-symbols-rounded">{"skip_previous"}</i>
                        <i class="material-symbols-rounded">{"play_circle"}</i>
                        <i class="material-symbols-rounded">{"skip_next"}</i>
                        <div class="top-panel__controls__shuffle-repeat">
                            <i class="material-symbols-rounded">{"shuffle"}</i>
                            <i class="material-symbols-rounded">{"repeat"}</i>
                        </div>
                    </div>
                    <div class="top-panel__controls__progress">
                        <input type="range" min="0" max="100" value="50" class="top-panel__controls__progress__slider" />
                    </div>
                </div>
                    // <div class="top-panel__controls__volume">
                    //     <i class="material-symbols-rounded">{"volume_up"}</i>
                    //     <input type="range" min="0" max="100" value="50" class="top-panel__controls__volume__slider" />
                    // </div>
                    // <div class="top-panel__controls__time">
                    //     <span>{"00:00"}</span>
                    //     <span>{"00:00"}</span>
                    // </div>
            </div>
        </div>
    }
}
