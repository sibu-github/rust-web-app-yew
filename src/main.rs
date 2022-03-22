use yew::prelude::*;
use components::row::Row;
use components::input::Input;


mod components;
mod splitter;


#[function_component(App)]
fn app() -> Html {
    let input_value = use_state(|| None);
    let current_input_row = use_state(|| 0_usize);
    let input_error = use_state(|| false);

    let board = use_state(|| {
        let v = vec![String::new(); 5];
        let v = vec![v; 7];
        v
    });

    let rows = {
        let board = board.clone();
        board.iter().enumerate().map(|(idx, b)| {
            let row_text = b.clone();
            html! {
                <Row row_text={row_text} selected_row_no={*current_input_row} row_no={idx}/>
            }
        }).collect::<Vec<_>>()
    };


    let on_change = {
        let input_value = input_value.clone();
        let current_input_row = current_input_row.clone();
        let board = board.clone();
        let input_error = input_error.clone();
        Callback::from(move|value: String|{
            log::info!("on_change: {:?}", &value);
            let chars = splitter::get_splitted(&value);
            log::info!("{:?}", chars);
            if chars.len() == 5 {
                let ref board_ref = *board;
                let mut board_vec = board_ref.clone();
                chars.iter().enumerate().for_each(|(idx, ch)| {
                    let b = board_vec.get_mut(*current_input_row).unwrap();
                    let bv = b.get_mut(idx).unwrap();
                    *bv = ch.to_string();
                });
                board.set(board_vec);
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
