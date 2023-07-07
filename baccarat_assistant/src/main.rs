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
    let counter = use_state(|| vec![8; 52]);
    let solution = use_solution((*counter).clone());

    // Initialize from
    {
        let number_of_decks = number_of_decks.clone();
        let counter = counter.clone();
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
                    Ok(v) => counter.set(v),
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
    let cards = convert_all_cards_to_html(&card_names, &counter);

    html! {
        <>
            {cards}
            <insights::InsightsDiv solution={solution} />

            <div id="control_buttons">
                <button id="reset" type="button">{"↻"}</button>
                <button id="undo" type="button">{"↶"}</button>
                <button id="redo" type="button">{"↷"}</button>
            </div>
        </>
    }
}

#[hook]
fn use_solution(counter: Vec<u32>) -> Solution {
    let solution: UseStateHandle<Solution> = use_state(|| Default::default());

    let solution_clone = solution.clone();
    let counter_clone = counter.clone();
    // TODO: Disable user to perform any operation until request is finished.
    let fetching = async move {
        let fetched_solution: Solution = gloo_net::http::Request::post(API_SOLVE)
            .json(&counter_clone)
            .unwrap()
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        solution_clone.set(fetched_solution);
    };

    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(fetching);
        },
        counter,
    );

    (*solution).clone()
}

fn convert_all_cards_to_html(
    card_names: &[[String; 13]; 4],
    counter: &UseStateHandle<Vec<u32>>,
) -> Html {
    // let rows = card_names.iter().map(convert_row_to_html).collect::<Html>();
    let mut rows = Vec::with_capacity(4);
    for (i, row) in card_names.iter().enumerate() {
        rows.push(convert_row_to_html(row, &counter, i));
    }
    html! {
        <div id="cards">
        {rows}
        </div>
    }
}

fn convert_row_to_html(
    row: &[String; 13],
    counter: &UseStateHandle<Vec<u32>>,
    row_number: usize,
) -> Html {
    let mut cards = Vec::with_capacity(13);
    for (i, name) in row.iter().enumerate() {
        cards.push(convert_name_to_html(name, counter, row_number * 13 + i));
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
    counter: &UseStateHandle<Vec<u32>>,
    card_index: usize,
) -> Html {
    let counter_clone = counter.clone();
    let onclick = move |_| {
        if counter_clone[card_index] == 0 {
            // TODO: Show hint message at frontend.
            return;
        }
        let mut new_counter = (*counter_clone).clone();
        new_counter[card_index] -= 1;
        counter_clone.set(new_counter.clone());
        if let Err(_) = <LocalStorage as Storage>::set(COUNTER, new_counter) {
            panic!("Cannot set local storage!");
        }
    };

    html! {
        <div class="card_info">
            <button type="button" onclick={Callback::from(onclick)}>{name}</button>
            {counter[card_index]}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
