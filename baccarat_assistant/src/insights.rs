use baccarat::calculation::Solution;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsInsights {
    pub solution: Solution,
}

#[function_component]
pub fn InsightsDiv(props: &PropsInsights) -> Html {
    let solution = &props.solution;
    let sm = &solution.sol_main;
    let sp = &solution.sol_pair;
    let sb = &solution.sol_bonus;
    let pu = &sb.p_player_bonus_unnatural;
    let bu = &sb.p_banker_bonus_unnatural;

    // Display EXs
    fn f(x: f64) -> String {
        format!("{:.2}%", x * 100.0)
    }

    // Display Probabilities
    fn g(x: f64) -> String {
        format!("{:.3}%", x * 100.0)
    }

    html! {
        <div id="insights">
            <table id="stat1"><thead><tr><th>{"Bet"}</th><th>{"Result"}</th><th>{"P"}</th><th>{"Total P"}</th><th>{"Ex"}</th></tr></thead><tbody><tr><td rowspan="2">{"Player Win"}</td><td>{"Player Win"}</td><td>{f(sm.p_player_win)}</td><td rowspan="2">{f(sm.p_player_win + sm.p_tie)}</td><td rowspan="2">{g(sm.ex_player_win)}</td></tr><tr><td>{"Tie"}</td><td>{f(sm.p_tie)}</td></tr><tr><td rowspan="2">{"Banker Win"}</td><td>{"Banker Win"}</td><td>{f(sm.p_banker_win)}</td><td rowspan="2">{f(sm.p_banker_win + sm.p_tie)}</td><td rowspan="2">{g(sm.ex_banker_win)}</td></tr><tr><td>{"Tie"}</td><td>{f(sm.p_tie)}</td></tr><tr><td>{"Tie"}</td><td>{"Tie"}</td><td>{f(sm.p_tie)}</td><td>{f(sm.p_tie)}</td><td>{g(sm.ex_tie)}</td></tr><tr><td>{"Player Pair"}</td><td>{"Player Pair"}</td><td>{f(sp.p_unsuit_pair)}</td><td>{f(sp.p_unsuit_pair)}</td><td>{g(sp.ex_unsuit_pair)}</td></tr><tr><td>{"Banker Pair"}</td><td>{"Banker Pair"}</td><td>{f(sp.p_unsuit_pair)}</td><td>{f(sp.p_unsuit_pair)}</td><td>{g(sp.ex_unsuit_pair)}</td></tr><tr><td>{"Either Pair"}</td><td>{"Either Pair"}</td><td>{f(sp.p_either_pair)}</td><td>{f(sp.p_either_pair)}</td><td>{g(sp.ex_either_pair)}</td></tr><tr><td rowspan="2">{"Perfect Pair"}</td><td>{"Either Perfect"}</td><td>{f(sp.p_suit_pair[0])}</td><td rowspan="2">{f(sp.p_suit_pair[0] + sp.p_suit_pair[1])}</td><td rowspan="2">{g(sp.ex_suit_pair)}</td></tr><tr><td>{"Both Perfect"}</td><td>{f(sp.p_suit_pair[1])}</td></tr></tbody></table>
            <table id="stat2"><thead><tr><th>{"Bet"}</th><th>{"Result"}</th><th>{"P"}</th><th>{"Total P"}</th><th>{"Ex"}</th></tr></thead><tbody><tr><td rowspan="8">{"Player Bonus"}</td><td>{"Bonus 4"}</td><td>{f(pu[0])}</td><td rowspan="8">{f(pu.iter().sum::<f64>() + sb.p_player_bonus_natural_win + sb.p_bonus_natural_tie)}</td><td rowspan="8">{g(sb.ex_player_bonus)}</td></tr><tr><td>{"Bonus 5"}</td><td>{f(pu[1])}</td></tr><tr><td>{"Bonus 6"}</td><td>{f(pu[2])}</td></tr><tr><td>{"Bonus 7"}</td><td>{f(pu[3])}</td></tr><tr><td>{"Bonus 8"}</td><td>{f(pu[4])}</td></tr><tr><td>{"Bonus 9"}</td><td>{f(pu[5])}</td></tr><tr><td>{"P Natural Win"}</td><td>{f(sb.p_player_bonus_natural_win)}</td></tr><tr><td>{"Natural Tie"}</td><td>{f(sb.p_bonus_natural_tie)}</td></tr><tr><td rowspan="8">{"Banker Bonus"}</td><td>{"Bonus 4"}</td><td>{f(bu[0])}</td><td rowspan="8">{f(bu.iter().sum::<f64>() + sb.p_banker_bonus_natural_win + sb.p_bonus_natural_tie)}</td><td rowspan="8">{g(sb.ex_banker_bonus)}</td></tr><tr><td>{"Bonus 5"}</td><td>{f(bu[1])}</td></tr><tr><td>{"Bonus 6"}</td><td>{f(bu[2])}</td></tr><tr><td>{"Bonus 7"}</td><td>{f(bu[3])}</td></tr><tr><td>{"Bonus 8"}</td><td>{f(bu[4])}</td></tr><tr><td>{"Bonus 9"}</td><td>{f(bu[5])}</td></tr><tr><td>{"B Natural Win"}</td><td>{f(sb.p_banker_bonus_natural_win)}</td></tr><tr><td>{"Natural Tie"}</td><td>{f(sb.p_bonus_natural_tie)}</td></tr></tbody></table>
        </div>
    }
}
