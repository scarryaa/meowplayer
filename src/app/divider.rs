use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[function_component(Divider)]
pub fn divider() -> Html {
    html! {
        <div class="divider"></div>
    }
}
