mod insights;

use baccarat::calculation::Solution;
use gloo_console;
use gloo_net;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures;
use yew::prelude::*;

const SUIT_EMOJIS: [&str; 4] = ["♦️", "♣️", "♥️", "♠️"];
const VALUE_CHARS: [char; 13] = [
    'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
];
const NUMBER_OF_DECKS: &str = "number_of_stacks";
const COUNTER: &str = "counter";

const API_SOLVE: &str = "/api/solve";

#[function_component]
fn App() -> Html {
    let number_of_decks = use_state(|| 8);
    let counter_display = use_state(|| vec![8; 52]);
    let hint = use_state(|| String::from(""));
    let is_requesting = use_state(|| false);
    let counter_try = use_state(|| vec![8; 52]);
    let retry_times = use_state(|| 1u32);
    let solution: UseStateHandle<Solution> = use_state(|| Default::default());

    // Effect for (counter_try, retry_times) -> (solution, is_requesting, hint, counter_display)
    {
        let solution = solution.clone();
        let counter_try = counter_try.clone();
        let counter_display = counter_display.clone();
        let retry_times = retry_times.clone();
        let hint = hint.clone();
        let is_requesting = is_requesting.clone();

        let counter_try_clone = counter_try.clone();
        let retry_times_clone = retry_times.clone();

        let fetching = async move {
            is_requesting.set(true);
            let response = gloo_net::http::Request::post(API_SOLVE)
                .json(&*counter_try)
                .unwrap()
                .send()
                .await;
            is_requesting.set(false);

            if let Err(ref e) = response {
                hint.set(format!(
                    "({}) Cannot get response from server: {}",
                    *retry_times,
                    e.to_string()
                ));
                return;
            }

            let response = response.unwrap();
            if !response.ok() {
                hint.set(format!(
                    "({}) Statuc {}: {}",
                    *retry_times,
                    response.status(),
                    response.status_text()
                ));
                return;
            }

            retry_times.set(0);

            let fetched_solution = response.json().await.unwrap();
            solution.set(fetched_solution);

            counter_display.set((*counter_try).clone());
            if let Err(_) = <LocalStorage as Storage>::set(COUNTER, (*counter_try).clone()) {
                panic!("Cannot set local storage!");
            }
        };

        let retry_times_inner = *retry_times_clone;
        use_effect_with_deps(
            move |_| {
                if *retry_times_clone == 0 {
                    return;
                }
                wasm_bindgen_futures::spawn_local(fetching);
            },
            ((*counter_try_clone).clone(), retry_times_inner),
        );
    }

    // Initialize from local storage.
    {
        let number_of_decks = number_of_decks.clone();
        let counter_display = counter_display.clone();
        let counter_try = counter_try.clone();
        use_effect_with_deps(
            move |_| {
                gloo_console::info!("Recovering from local storage");
                let current_number_of_decks;
                match <LocalStorage as Storage>::get::<u32>(NUMBER_OF_DECKS) {
                    Ok(v) => {
                        number_of_decks.set(v);
                        current_number_of_decks = v;
                    }
                    Err(_) => {
                        let msg = format!(
                            "'{}' not found in local storage. Default (8) is used",
                            NUMBER_OF_DECKS
                        );
                        gloo_console::warn!(msg);
                        current_number_of_decks = 8;
                        if let Err(_) = <LocalStorage as Storage>::set(NUMBER_OF_DECKS, 8) {
                            panic!("Cannot set local storage!");
                        }
                    }
                }

                match <LocalStorage as Storage>::get::<Vec<u32>>(COUNTER) {
                    Ok(v) => {
                        counter_display.set(v.clone());
                        counter_try.set(v);
                    }
                    Err(_) => {
                        let msg =
                            format!("'{}' not found in local storage. Default is used", COUNTER);
                        gloo_console::warn!(msg);
                        if let Err(_) = <LocalStorage as Storage>::set(
                            COUNTER,
                            vec![current_number_of_decks; 52],
                        ) {
                            panic!("Cannot set local storage!");
                        }
                    }
                }
            },
            (),
        );
    }

    let mut card_names: [[String; 13]; 4] = Default::default();
    for i in 0..4 {
        for j in 0..13 {
            card_names[i][j] = format!("{}{}", SUIT_EMOJIS[i], VALUE_CHARS[j]);
        }
    }
    let cards = convert_all_cards_to_html(
        &card_names,
        counter_display.clone(),
        counter_try.clone(),
        hint.clone(),
        is_requesting.clone(),
        retry_times.clone(),
    );

    let (hint_msg, hint_cls) = if hint.len() == 0 {
        (String::from("x"), "invisible")
    } else {
        ((*hint).clone(), "")
    };

    html! {
        <>
            {cards}
            <div id="hint" class={hint_cls}>{hint_msg}</div>
            <insights::InsightsDiv solution={(*solution).clone()} />

            <div id="control_buttons">
                <button id="reset" type="button">{"↻"}</button>
                <button id="undo" type="button">{"↶"}</button>
                <button id="redo" type="button">{"↷"}</button>
            </div>
        </>
    }
}

fn convert_all_cards_to_html(
    card_names: &[[String; 13]; 4],
    counter_display: UseStateHandle<Vec<u32>>,
    counter_try: UseStateHandle<Vec<u32>>,
    hint: UseStateHandle<String>,
    is_requesting: UseStateHandle<bool>,
    retry_times: UseStateHandle<u32>,
) -> Html {
    let mut rows = Vec::with_capacity(4);
    for (i, row) in card_names.iter().enumerate() {
        rows.push(convert_row_to_html(
            row,
            counter_display.clone(),
            counter_try.clone(),
            i,
            hint.clone(),
            is_requesting.clone(),
            retry_times.clone(),
        ));
    }
    html! {
        <div id="cards">
        {rows}
        </div>
    }
}

fn convert_row_to_html(
    row: &[String; 13],
    counter_display: UseStateHandle<Vec<u32>>,
    counter_try: UseStateHandle<Vec<u32>>,
    row_number: usize,
    hint: UseStateHandle<String>,
    is_requesting: UseStateHandle<bool>,
    retry_times: UseStateHandle<u32>,
) -> Html {
    let mut cards = Vec::with_capacity(13);
    for (i, name) in row.iter().enumerate() {
        cards.push(convert_name_to_html(
            name,
            counter_display.clone(),
            counter_try.clone(),
            row_number * 13 + i,
            hint.clone(),
            is_requesting.clone(),
            retry_times.clone(),
        ));
    }

    let css_class = if row[0].starts_with(SUIT_EMOJIS[0]) || row[0].starts_with(SUIT_EMOJIS[2]) {
        "card_row red"
    } else {
        "card_row"
    };
    html! {
        <div class={css_class}>
        {cards}
        </div>
    }
}

fn convert_name_to_html(
    name: &String,
    counter_display: UseStateHandle<Vec<u32>>,
    counter_try: UseStateHandle<Vec<u32>>,
    card_index: usize,
    hint: UseStateHandle<String>,
    is_requesting: UseStateHandle<bool>,
    retry_times: UseStateHandle<u32>,
) -> Html {
    let already_zero = format!("The number of {} is already ZERO!!", name);
    let requesting_error = String::from("Waiting for server response. Can't send request again.");
    let counter_display_preserved = counter_display.clone();
    let onclick = move |_| {
        if *is_requesting {
            hint.set(requesting_error.clone());
            return;
        }
        if counter_display[card_index] == 0 {
            hint.set(already_zero.clone());
            return;
        }
        if hint.len() != 0 {
            hint.set(String::from(""));
        }
        let mut new_counter = (*counter_display).clone();
        new_counter[card_index] -= 1;
        counter_try.set(new_counter);
        retry_times.set(*retry_times + 1);
    };

    html! {
        <div class="card_info">
            <button type="button" onclick={Callback::from(onclick)}>{name}</button>
            {counter_display_preserved[card_index]}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
