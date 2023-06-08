use yew::prelude::*;

#[function_component(TitleBar)]
pub fn title_bar() -> Html {
    html! {
        <>
            <div class="resize-margin"/>
            <div class="titlebar" data-tauri-drag-region="true">
                <div class="titlebar__buttons">
                    <div class="titlebar__buttons__button titlebar__buttons__minimize">
                        <i class="material-symbols-rounded">{"minimize"}</i>
                    </div>
                    <div class="titlebar__buttons__button titlebar__buttons__maximize">
                        <i class="material-symbols-rounded">{"maximize"}</i>
                    </div>
                    <div class="titlebar__buttons__button titlebar__buttons__close">
                        <i class="material-symbols-rounded">{"close"}</i>
                    </div>
                </div>
            </div>
        </>
    }
}
