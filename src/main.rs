use components::input::Input;
use components::row::Row;
use components::row::RowData;
use yew::prelude::*;

mod components;
mod splitter;

struct Board {
    rows_data: Vec<RowData>,
}

impl Board {
    fn new() -> Self {
        let no_of_rows = Self::get_no_of_rows();
        let row_size = Self::get_row_size();
        let rows_data = (0..no_of_rows)
            .map(|_| RowData::new(row_size))
            .collect::<Vec<_>>();
        Self {
            rows_data
        }
    }

    fn get_row_size() -> usize {
        5
    }

    fn get_no_of_rows() -> usize {
        7
    }

}

#[function_component(App)]
fn app() -> Html {
    let input_value = use_state(|| None);
    let current_input_row = use_state(|| 0_usize);
    let input_error = use_state(|| false);
    let board = use_state(|| Board::new());

    let rows = {
        let board = board.clone();
        board
            .rows_data
            .iter()
            .enumerate()
            .map(|(idx, b)| {
                let data = b.clone();
                html! {
                    <Row data={data} selected_row_no={*current_input_row} row_no={idx}/>
                }
            })
            .collect::<Vec<_>>()
    };

    let on_change = {
        let input_value = input_value.clone();
        let current_input_row = current_input_row.clone();
        let board = board.clone();
        let input_error = input_error.clone();
        Callback::from(move |value: String| {
            log::info!("on_change: {:?}", &value);
            let chars = splitter::get_splitted(&value);
            log::info!("{:?}", chars);
            if chars.len() == Board::get_row_size() {
                let mut rows_data = board.rows_data.clone();
                let row_data = rows_data.get_mut(*current_input_row).unwrap();
                row_data.update_cell_text(&chars);
                let new_board = Board {rows_data };
                board.set(new_board);
                input_error.set(false);
                input_value.set(None);
                let val = *current_input_row + 1;
                current_input_row.set(val);
            } else {
                input_error.set(true);
                input_value.set(None);
            }
        })
    };

    let value = {
        let input_value = input_value.clone();
        let s = String::new();
        input_value.as_ref().unwrap_or(&s).to_string()
    };

    html! {
        <div class="app">
            <h1>{ "শব্দ খোঁজ" }</h1>
            <div class="box">
                {for rows}
            </div>
            <Input
                value={value}
                on_change={on_change.clone()}
                show_error={*input_error}
            />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
