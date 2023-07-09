use gloo_console::error;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsDecksEdit {
    pub on_confirm: Callback<u32>,
    pub initial_num: u32,
}

#[function_component]
pub fn DecksEdit(props: &PropsDecksEdit) -> Html {
    let new_decks = use_state(|| props.initial_num);

    let onchange_input = {
        let new_decks = new_decks.clone();
        move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let value = input.value();
                if let Ok(value) = value.parse::<u32>() {
                    new_decks.set(value);
                } else {
                    error!("Cannot parse u32: ", value);
                }
            }
        }
    };

    let onclick_confirm = {
        let new_decks = new_decks.clone();
        let on_confirm = props.on_confirm.clone();
        move |_| {
            on_confirm.emit(*new_decks);
        }
    };

    html! {
        <div id="decks_edit_bg" class="popup_bg">
        <div id="decks_edit">
        <div id="change_decks">
        {"Change number of decks: "}
        <input type="number" value={(*new_decks).to_string()} onchange={Callback::from(onchange_input)} />
        </div>
        <button id="confirm_change_decks" type="button" onclick={Callback::from(onclick_confirm)}>{"Confirm"}</button>
        </div>
        </div>
    }
}
