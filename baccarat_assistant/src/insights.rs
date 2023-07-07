use baccarat::calculation::Solution;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsInsights {
    pub solution: Solution,
}

#[function_component]
pub fn InsightsDiv(props: &PropsInsights) -> Html {
    html! {
        <div id="insights">
            {start_div(&props.solution)}
            {suggestion_div(&props.solution)}
        </div>
    }
}

pub fn start_div(solution: &Solution) -> Html {
    html! {
    <table>
    <thead>
      <tr>
        <th>{"Bet"}</th>
        <th>{"Result"}</th>
        <th>{"Probability"}</th>
        <th>{"Total P"}</th>
        <th>{"Expectation"}</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td rowspan="2">{"Player Win"}</td>
        <td>{"Player Win"}</td>
        <td></td>
        <td rowspan="2"></td>
        <td rowspan="2"></td>
      </tr>
      <tr>
        <td>{"Tie"}</td>
        <td></td>
      </tr>
      <tr>
        <td rowspan="2">{"Banker Win"}</td>
        <td>{"Banker Win"}</td>
        <td></td>
        <td rowspan="2"></td>
        <td rowspan="2"></td>
      </tr>
      <tr>
        <td>{"Tie"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Tie"}</td>
        <td>{"Tie"}</td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>{"Player Pair"}</td>
        <td>{"Player Pair"}</td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>{"Banker Pair"}</td>
        <td>{"Banker Pair"}</td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td rowspan="2">{"Perfect Pair"}</td>
        <td>{"Either Perfect"}</td>
        <td></td>
        <td rowspan="2"></td>
        <td rowspan="2"></td>
      </tr>
      <tr>
        <td>{"Both Perfect"}</td>
        <td></td>
      </tr>
      <tr>
        <td rowspan="8">{"Player Bonus"}</td>
        <td>{"Bonus 4"}</td>
        <td></td>
        <td rowspan="8"></td>
        <td rowspan="8"></td>
      </tr>
      <tr>
        <td>{"Bonus 5"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 6"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 7"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 8"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 9"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Natural Win"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Natural Tie"}</td>
        <td></td>
      </tr>
      <tr>
        <td rowspan="8">{"Banker Bonus"}</td>
        <td>{"Bonus 4"}</td>
        <td></td>
        <td rowspan="8"></td>
        <td rowspan="8"></td>
      </tr>
      <tr>
        <td>{"Bonus 5"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 6"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 7"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 8"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Bonus 9"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Natural Win"}</td>
        <td></td>
      </tr>
      <tr>
        <td>{"Natural Tie"}</td>
        <td></td>
      </tr>
    </tbody>
    </table>
    }
}

fn suggestion_div(solution: &Solution) -> Html {
    html! {
        <div id="suggestion">
            {format!("{:#?}", solution)}
        </div>
    }
}
