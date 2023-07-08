use baccarat::calculation::Solution;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsInsights {
    pub solution: Solution,
}

// Probability And Ex. In short, PEX.
#[derive(Debug, Clone)]
struct Pex(&'static str, f64, f64);

#[function_component]
pub fn InsightsDiv(props: &PropsInsights) -> Html {
    let solution = &props.solution;
    let sm = &solution.sol_main;
    let sp = &solution.sol_pair;
    let sb = &solution.sol_bonus;
    let pu = &sb.p_player_bonus_unnatural;
    let bu = &sb.p_banker_bonus_unnatural;

    let bets = vec![
        Pex("Player Win", sm.p_player_win + sm.p_tie, sm.ex_player_win),
        Pex("Banker Win", sm.p_banker_win + sm.p_tie, sm.ex_banker_win),
        Pex("Tie", sm.p_tie, sm.ex_tie),
        Pex("Player Pair", sp.p_unsuit_pair, sp.ex_unsuit_pair),
        Pex("Banker Pair", sp.p_unsuit_pair, sp.ex_unsuit_pair),
        Pex("Either Pair", sp.p_either_pair, sp.ex_either_pair),
        Pex(
            "Perfect Pair",
            sp.p_suit_pair[0] + sp.p_suit_pair[1],
            sp.ex_suit_pair,
        ),
        Pex(
            "Player Bonus",
            sb.p_player_bonus_unnatural.iter().sum::<f64>()
                + sb.p_player_bonus_natural_win
                + sb.p_bonus_natural_tie,
            sb.ex_player_bonus,
        ),
        Pex(
            "Banker Bonus",
            sb.p_banker_bonus_unnatural.iter().sum::<f64>()
                + sb.p_banker_bonus_natural_win
                + sb.p_bonus_natural_tie,
            sb.ex_banker_bonus,
        ),
        Pex("No Bet", 1.0, 0.0),
    ];

    let mut bets_main_p = bets[0..3].to_vec();
    let mut bets_main_ex = bets_main_p.clone();
    bets_main_ex.push(bets.last().unwrap().clone());
    bets_main_ex.sort_by(|x, y| y.2.partial_cmp(&x.2).unwrap());
    bets_main_p.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

    let mut bets_side_p = bets[3..9].to_vec();
    let mut bets_side_ex = bets_side_p.clone();
    bets_side_ex.push(bets.last().unwrap().clone());
    bets_side_ex.sort_by(|x, y| y.2.partial_cmp(&x.2).unwrap());
    bets_side_p.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

    html! {
        <div id="insights">
            <table id="suggestion">
                <tr> <th>{"Goal"}</th> <th>{"1st"}</th> <th>{"2nd"}</th> <th>{"3rd"}</th> </tr>
                {best_suggestion_row(solution, &bets)}
                {suggestion_row("Main bet (EX)", &bets_main_ex)}
                {suggestion_row("Side bet (EX)", &bets_side_ex)}
                {suggestion_row("Main bet (P)", &bets_main_p)}
                {suggestion_row("Side bet (P)", &bets_side_p)}
            </table>
            <table id="stat1"><thead><tr><th>{"Bet"}</th><th>{"Result"}</th><th>{"P"}</th><th>{"Total P"}</th><th>{"Ex"}</th></tr></thead><tbody><tr><td rowspan="2">{"Player Win"}</td><td>{"Player Win"}</td><td>{f(sm.p_player_win)}</td><td rowspan="2">{f(bets[0].1)}</td><td rowspan="2">{g(bets[0].2)}</td></tr><tr><td>{"Tie"}</td><td>{f(sm.p_tie)}</td></tr><tr><td rowspan="2">{"Banker Win"}</td><td>{"Banker Win"}</td><td>{f(sm.p_banker_win)}</td><td rowspan="2">{f(bets[1].1)}</td><td rowspan="2">{g(bets[1].2)}</td></tr><tr><td>{"Tie"}</td><td>{f(sm.p_tie)}</td></tr><tr><td>{"Tie"}</td><td>{"Tie"}</td><td>{f(sm.p_tie)}</td><td>{f(bets[2].1)}</td><td>{g(bets[2].2)}</td></tr><tr><td>{"Player Pair"}</td><td>{"Player Pair"}</td><td>{f(sp.p_unsuit_pair)}</td><td>{f(bets[3].1)}</td><td>{g(bets[3].2)}</td></tr><tr><td>{"Banker Pair"}</td><td>{"Banker Pair"}</td><td>{f(sp.p_unsuit_pair)}</td><td>{f(bets[4].1)}</td><td>{g(bets[4].2)}</td></tr><tr><td>{"Either Pair"}</td><td>{"Either Pair"}</td><td>{f(sp.p_either_pair)}</td><td>{f(bets[5].1)}</td><td>{g(bets[5].2)}</td></tr><tr><td rowspan="2">{"Perfect Pair"}</td><td>{"Either Perfect"}</td><td>{f(sp.p_suit_pair[0])}</td><td rowspan="2">{f(bets[6].1)}</td><td rowspan="2">{g(bets[6].2)}</td></tr><tr><td>{"Both Perfect"}</td><td>{f(sp.p_suit_pair[1])}</td></tr></tbody></table>
            <table id="stat2"><thead><tr><th>{"Bet"}</th><th>{"Result"}</th><th>{"P"}</th><th>{"Total P"}</th><th>{"Ex"}</th></tr></thead><tbody><tr><td rowspan="8">{"Player Bonus"}</td><td>{"Bonus 4"}</td><td>{f(pu[0])}</td><td rowspan="8">{f(bets[7].1)}</td><td rowspan="8">{g(bets[7].2)}</td></tr><tr><td>{"Bonus 5"}</td><td>{f(pu[1])}</td></tr><tr><td>{"Bonus 6"}</td><td>{f(pu[2])}</td></tr><tr><td>{"Bonus 7"}</td><td>{f(pu[3])}</td></tr><tr><td>{"Bonus 8"}</td><td>{f(pu[4])}</td></tr><tr><td>{"Bonus 9"}</td><td>{f(pu[5])}</td></tr><tr><td>{"P Natural Win"}</td><td>{f(sb.p_player_bonus_natural_win)}</td></tr><tr><td>{"Natural Tie"}</td><td>{f(sb.p_bonus_natural_tie)}</td></tr><tr><td rowspan="8">{"Banker Bonus"}</td><td>{"Bonus 4"}</td><td>{f(bu[0])}</td><td rowspan="8">{f(bets[8].1)}</td><td rowspan="8">{g(bets[8].2)}</td></tr><tr><td>{"Bonus 5"}</td><td>{f(bu[1])}</td></tr><tr><td>{"Bonus 6"}</td><td>{f(bu[2])}</td></tr><tr><td>{"Bonus 7"}</td><td>{f(bu[3])}</td></tr><tr><td>{"Bonus 8"}</td><td>{f(bu[4])}</td></tr><tr><td>{"Bonus 9"}</td><td>{f(bu[5])}</td></tr><tr><td>{"B Natural Win"}</td><td>{f(sb.p_banker_bonus_natural_win)}</td></tr><tr><td>{"Natural Tie"}</td><td>{f(sb.p_bonus_natural_tie)}</td></tr></tbody></table>
        </div>
    }
}

fn best_suggestion_row(solution: &Solution, bets: &[Pex]) -> Html {
    let max_ex = solution.get_best_main_side_bet(0.0);

    fn hands_bet_to_pex_index(hands_bet: baccarat::game::HandsBet) -> usize {
        match hands_bet {
            baccarat::game::HandsBet::PlayerWin => 0,
            baccarat::game::HandsBet::BankerWin => 1,
            baccarat::game::HandsBet::Tie => 2,
            baccarat::game::HandsBet::PlayerUnsuitPair => 3,
            baccarat::game::HandsBet::BankerUnsuitPair => 4,
            baccarat::game::HandsBet::EitherPair => 5,
            baccarat::game::HandsBet::PerfectPair => 6,
            baccarat::game::HandsBet::PlayerBonus => 7,
            baccarat::game::HandsBet::BankerBonus => 8,
            baccarat::game::HandsBet::PlaceHolder => 9,
        }
    }

    let main_pex = &bets[hands_bet_to_pex_index(max_ex.0 .0)];
    let side_pex = &bets[hands_bet_to_pex_index(max_ex.1 .0)];
    let (content_bet, content_ex) = {
        if side_pex.2 < main_pex.2 && main_pex.2 > 0.0 {
            (
                format!("<{}>", main_pex.0),
                format!(" [Ex: {}]", g(main_pex.2)),
            )
        } else if side_pex.2 > main_pex.2 && 2.0 * main_pex.2 + side_pex.2 > 0.0 {
            (
                format!("<{}>(2x) + <{}>(1x)", main_pex.0, side_pex.0),
                format!(" [{:.3} / 300]", 2.0 * main_pex.2 + side_pex.2),
            )
        } else {
            (String::from("No Bet"), String::from(""))
        }
    };

    html! {
        <tr>
            <td>{"Best Ex"}</td>
            <td colspan="3"><strong>{content_bet}</strong>{content_ex}</td>
        </tr>
    }
}

fn suggestion_row(name: &str, bets: &[Pex]) -> Html {
    fn h(bet: &Pex) -> Html {
        html! {
            <td><strong>{format!("<{}>", bet.0)}</strong><br/>{format!("[{}, {}]", f(bet.1), g(bet.2))}</td>
        }
    }
    html! {
        <tr>
            <td>{name}</td>
            {h(&bets[0])}
            {h(&bets[1])}
            {h(&bets[2])}
        </tr>
    }
}

// Display Probabilities
fn f(x: f64) -> String {
    format!("{:.3}%", x * 100.0)
}

// Display EXs
fn g(x: f64) -> String {
    format!("{:.3}%", x * 100.0)
}
