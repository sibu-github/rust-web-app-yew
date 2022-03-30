use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CellData {
    pub txt: String,
    pub is_fully_matched: bool,
    pub is_partially_matched_correct_pos: bool,
    pub is_partially_matched_wrong_pos: bool,
    pub is_fully_wrong: bool,
}

impl CellData {
    pub fn new() -> Self {
        Self {
            txt: String::new(),
            is_fully_matched: false,
            is_partially_matched_correct_pos: false,
            is_partially_matched_wrong_pos: false,
            is_fully_wrong: false,
        }
    }

}

#[derive(Clone, Properties, PartialEq)]
pub struct CellProps {
    pub data: CellData,
}


#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let text = props.data.txt.to_owned();
    let style = {
        if props.data.is_fully_matched { "color: green;" }
        else if props.data.is_partially_matched_correct_pos { "color: yellow;" }
        else if props.data.is_partially_matched_wrong_pos { "color: blue;" }
        else if props.data.is_fully_wrong { "color: red;" }
        else { "color: white;" }
    };

    html! {
        <div class="cell" style={style}>
            {text}
        </div>
    }
}
