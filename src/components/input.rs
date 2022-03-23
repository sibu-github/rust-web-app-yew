use reqwasm::http::Request;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use regex::Regex;


fn is_english(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    }
    RE.is_match(text)
}


#[derive(Debug, Serialize, Deserialize)]
struct CandidateType {
    candidate_type: Vec<u64>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum InnerData {
    Text(String),
    TextArray(Vec<String>),
    NumberArray(Vec<u64>),
    CandidateDataType(CandidateType)
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Data {
    Text(String),
    InnerDataType(Vec<Vec<InnerData>>),
}

#[derive(Clone, Properties, PartialEq)]
pub struct InputProps {
    pub value: String,
    pub on_change: Callback<String>,
    pub show_error: bool,
}


#[function_component(Input)]
pub fn input(input_props: &InputProps) -> Html {
    let value = use_state(|| String::new());
    let datalist = use_state(|| {
        Vec::<String>::new()
    });

    let onkeyup = {
        let value = value.clone();
        let datalist = datalist.clone();
        let on_change = input_props.on_change.clone();
        move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let v = input.value().to_string();
            value.set(v.to_owned());
            if e.key() != "Enter"{
                return;
            }

            if !is_english(&v){
                log::info!("Input is not in english");
                on_change.emit(v.to_owned());
                return;
            }

            let datalist = datalist.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("https://inputtools.google.com/request?text={}&itc=bn-t-i0-und&num=5&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", &v);
                let r = Request::post(&url).send().await;
                if r.is_err() {
                    log::error!("{:?}", r.err().unwrap());
                    return;
                }
                let r = r.unwrap();
                let rs = r.json::<Vec<Data>>().await;
                if rs.is_err() {
                    log::error!("{:?}", rs.err().unwrap()); 
                    return;
                }
                let rs = rs.unwrap();
                // log::info!("{:?}", &rs);
                if rs.len() < 2 {
                    log::info!("API response is not in proper structure");
                    return;
                }
                if let Data::Text(s) = &rs[0] {
                    if s != "SUCCESS" {
                        log::info!("API response is not in proper structure");
                        return;
                    }
                }
                if let Data::InnerDataType(v) = &rs[1] {
                    if v.len() < 1 {
                        return;
                    }
                    let v = &v[0];
                    if let InnerData::TextArray(v) = &v[1] {
                        log::info!("{:?}", &v);
                        let dl = v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                        datalist.set(dl);
                    }
                }
            });
        }
    };


    let error_div = if input_props.show_error {
        html! {
            <div class="input-err">{"শুধুমাত্র ৫ অক্ষরের ১টি বাংলা শব্দ প্রয়োগ করা যাবে।"}</div>
        }
    } else {
       html!{ <div></div> }
    };


    let data_list_items = {
        datalist.iter().map(|itm|{
            let onclick = {
                let value = value.clone();
                let s = itm.to_owned();
                let datalist = datalist.clone();
                let on_change = input_props.on_change.clone();
                Callback::from(move |_| {
                    value.set(s.to_owned());
                    datalist.set(vec![]);
                    on_change.emit(s.to_owned());
                })
            };

            let s = itm.to_owned();
            html! { 
                <li onclick={onclick}> 
                    {s} 
                </li>
            }
        }).collect::<Vec<_>>()
    };


    html! {
        <div class="input-wrapper">
            {error_div}
            <input 
                type="text" 
                value={value.clone().to_string()}
                onkeyup={onkeyup}
                list="CktPlayers"
                placeholder="আপনার নির্বাচিত শব্দটি লিখে ENTER প্রেস করুন..."/>
            <div class="datalist">
                <ul>
                    {for data_list_items}
                </ul>
            </div>
        </div>
    }
}