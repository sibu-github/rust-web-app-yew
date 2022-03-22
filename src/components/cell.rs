use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct CellProps {
    pub cell_text: String,
}

#[function_component(Cell)]
pub fn cell(cell_props: &CellProps) -> Html {
    let text = cell_props.cell_text.to_owned();
    html! {
        <div class="cell">
            {text}
        </div>
    }
}