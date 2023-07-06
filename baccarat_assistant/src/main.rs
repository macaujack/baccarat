use yew::prelude::*;

const SUIT_EMOJIS: [&str; 4] = ["♦️", "♣️", "♥️", "♠️"];
const VALUE_CHARS: [char; 13] = [
    'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
];

#[function_component]
fn App() -> Html {
    let mut card_names: [[String; 13]; 4] = Default::default();
    for i in 0..4 {
        for j in 0..13 {
            card_names[i][j] = format!("{}{}", SUIT_EMOJIS[i], VALUE_CHARS[j]);
        }
    }
    let cards = convert_all_cards_to_html(&card_names);

    html! {
        <>
            {cards}
        </>
    }
}

fn convert_all_cards_to_html(card_names: &[[String; 13]; 4]) -> Html {
    let rows = card_names.iter().map(convert_row_to_html).collect::<Html>();
    html! {
        <div id="cards">
        {rows}
        </div>
    }
}

fn convert_row_to_html(row: &[String; 13]) -> Html {
    let cards = row.iter().map(convert_name_to_html).collect::<Html>();
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

fn convert_name_to_html(name: &String) -> Html {
    html! {
        <div class="card_info">
            <button type="button">{name}</button>
            {"haha"}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
