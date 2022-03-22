use yew::prelude::*;
use super::cell::Cell;



#[derive(Clone, Properties, PartialEq)]
pub struct RowProps {
    pub row_text: Vec<String>,
    pub selected_row_no: usize,
    pub row_no: usize,
}

#[function_component(Row)]
pub fn row(row_props: &RowProps) -> Html {
    let row_text = row_props.row_text.clone();
    let cells = row_text.iter().map(|s|{
        let s = s.to_owned();
        html! {
            <Cell cell_text={s}/>
        }
    }).collect::<Vec<_>>();

    let class= if row_props.selected_row_no == row_props.row_no { "row selected" } else { "row " };


    html! {
        <div class={class}>
            {for cells}
        </div>
    }
}