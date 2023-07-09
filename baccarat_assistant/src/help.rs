use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsHelp {
    pub on_close: Callback<()>,
}

#[function_component]
pub fn HelpDiv(props: &PropsHelp) -> Html {
    let onclick_help_bg = {
        let on_close = props.on_close.clone();
        move |_| {
            on_close.emit(());
        }
    };

    html! {
        <div id="help_bg" class="popup_bg" onclick={Callback::from(onclick_help_bg)}>
        <div id="help_content">
        <h1>{"Help"}</h1>
        <p>{"(Click anywhere to exit the help page.)"}</p>
        <p>{"This Web APP mainly consists of 3 parts: card counter, control buttons, tables of insights. If you encounter any issue, please contact Jack Y. <seigino.mikata@outlook.com>."}</p>
        <h2>{"Card counter"}</h2>
        <p>{"There are 52 buttons, with each standing for a card. Below each card counter button, there is a number standing for the number of this card remained in the shoe."}</p>
        <p>{"When you click a button, its number will be decreased by 1. This means a card is dealt from the shoe. You can see every time you deal a card, the tables at the bottom will slightly change."}</p>
        <h2>{"Control buttons"}</h2>
        <p>{"There are 4 control buttons. They are '↶' (Undo), '↷' (Redo), '↻' (Reset) and '?' (Help) respectively. Undo will cancel the most recent card dealing. Redo cancels undo. You can undo/redo for at most 99 times. Reset will literally reset the shoe to initial state, with the same number of cards for each card."}</p>
        <h2>{"Tables of insights"}</h2>
        <p>{"There are 3 tables here. The first table gives suggesstion on what you should bet (or more probably no bet at all). The second and third table are actually two horizontally separated parts of a big table, showing each bet's probability and expectation."}</p>
        <p>{"If all you want is to maximize your profit, you should only focus on the first row of the first table, which gives you the best bet (perhaps no bet) based on expectation. "}<strong>{"Note that because you can hardly get a positive expectation from the main bets in Baccarat, you will hardly see it suggesting you to only bet on main bet. It always suggests you to bet on main bet along with side bet."}</strong>{" Here when calculating expectation, we assume that side bet can be at most half of main bet."}</p>
        <p>{"If you feel lucky or don't want to follow math, the first table also give you some suggestions. It gives you suggestions on main bets and side bets, based on both probability and expectation. The 3 columns called '1st', '2nd', '3rd' give you the best 3 suggestions. For example, let's say you just want to bet on some side bets today, and you just want to win side bets and don't care about main bet. In this case, you can focus on the 'Side bet (P)' row. The 'P' indicates the 3 best suggestions are ordered by probability, instead of expectation. Since total side bet cannot exeed half of main bet, you have to bet double on main bet in order to bet on side bet. When betting on main bet, you can refer to the 'Main bet (EX)' row, which gives you the best expectations among 3 main bets (plus no bet)."}</p>
        <p>{"The other 2 tables together give the probability and expectation for each bet. Most time you won't use them, but out of curiosity, you can observe how probabilities and expectations change with more and more cards dealt. And finally you can conclude that it's really really really hard to make money by playing Baccarat, a game designed to make you think you can make money :)"}</p>
        </div>
        </div>
    }
}
