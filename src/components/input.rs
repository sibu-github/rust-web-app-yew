use yew::prelude::*;
use web_sys::HtmlInputElement;


#[derive(Clone, Properties, PartialEq)]
pub struct InputProps {
    pub value: String,
    pub on_change: Callback<String>,
    pub show_error: bool,
}

#[function_component(Input)]
pub fn input(input_props: &InputProps) -> Html {
    let onchange = {
        let on_change = input_props.on_change.clone();

        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().to_string();
            on_change.emit(value);
        }
    };


    let error_div = if input_props.show_error {
        html! {
            <div class="input-err">{"শুধুমাত্র ৫ অক্ষরের ১টি বাংলা শব্দ প্রয়োগ করা যাবে।"}</div>
        }
    } else {
       html!{ <div></div> }
    };


    html! {
        <div class="input-wrapper">
            {error_div}
            <input 
                type="text" 
                value={input_props.value.to_owned()}
                onchange={onchange}
                placeholder="আপনার নির্বাচিত শব্দটি লিখে ENTER প্রেস করুন..."/>
        </div>
    }
}