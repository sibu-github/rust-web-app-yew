use super::cell::Cell;
use super::cell::CellData;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RowData {
    pub cell_data: Vec<CellData>,
}

impl RowData {
    pub fn new(row_size: usize) -> Self {
        let cell_data = (0..row_size).map(|_| CellData::new()).collect::<Vec<_>>();
        Self { cell_data }
    }


    pub fn update_cell_text(&mut self, chars: &Vec<String>) {
        for (i, s) in chars.iter().enumerate() {
            if let Some(v) = self.cell_data.get_mut(i) {
                v.txt = s.to_owned();
            }
        }
    }
}





#[derive(Debug, Clone, Properties, PartialEq)]
pub struct RowProps {
    pub data: RowData,
    pub row_no: usize,
    pub selected_row_no: usize
}


#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {

    let cells = props.data.cell_data.iter().map(|d| {
        let d = d.clone();
        html! {
            <Cell data={d} />
        }
    }).collect::<Vec<_>>();

    let class = if props.selected_row_no == props.row_no {
        "row selected"
    } else {
        "row "
    };

    html! {
        <div class={class}>
            {for cells}
        </div>
    }
}
