use std::vec;

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PositionFieldProps {
    pub hint: String,
}

#[function_component(PositionField)]
pub fn positionfield(props: &PositionFieldProps) -> Html {
    html! {
        <input type="text" class="form-control" placeholder={props.hint.clone()} />
    }
}

#[derive(Properties, PartialEq, Clone, Serialize, Deserialize)]
struct PositionPayload {
    pub title: String,
}

/**
 * A table of skills
 */
#[derive(Properties, PartialEq, Clone)]
pub struct SkillTableProps {
    pub skills: Vec<(String, bool)>,
}
#[function_component(SkillTable)]
pub fn skilltable(props: &SkillTableProps) -> Html {
    let skills = use_state(|| props.skills.clone());
    let onchanged = {
        let skills = skills.clone();
        Callback::from(move |e: Event| {
            // get the elements id and checked value
            let target = e.target().unwrap();
            let target = target.dyn_into::<HtmlInputElement>().unwrap();
            let id = target.id();
            let checked = target.checked();

            let mut vec: Vec<(String, bool)> = vec![];
            let skills = skills.clone();
            for i in 0..skills.len() {
                vec.push((skills[i].0.clone(), skills[i].1));
                if vec[i].0 == id {
                    vec[i].1 = checked;
                }
            }
            vec.sort_by(|a, b| a.1.cmp(&b.1));
            skills.clone().set(Vec::clone(&vec));
        })
    };
    html! {
        <table class="table-auto">
            <thead>
                <tr>
                    <th class="px-4 py-2">{"Skills"}</th>
                    <th class="px-4 py-2">{"Did"}</th>
                </tr>
            </thead>
            <tbody>
                {for skills.iter().map( |skill| html! {
                    <tr>
                        <td class="border px-4 py-2">{skill.0.clone()}</td>
                        <td class="border px-4 py-2">
                            <input type="checkbox" id={skill.0.clone()} class="form-checkbox h-5 w-5 text-slate-600" checked={skill.1} onchange={onchanged.clone()}/>
                        </td>
                    </tr>
                })}
            </tbody>
        </table>
    }
}

#[function_component]
fn App() -> Html {
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        // let greeting = String::from("Hi there");
        wasm_bindgen_futures::spawn_local(async move {
            let resp = Request::post("http://localhost:8081/test")
                .json(&PositionPayload {
                    title: "test".to_string(),
                })
                .unwrap()
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 200);
            web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", resp)));
        });
    });

    // dummy table data
    let dummy_data = SkillTableProps {
        skills: vec![
            ("C++".to_string(), false),
            ("Rust".to_string(), false),
            ("Python".to_string(), false),
            ("Javascript".to_string(), false),
        ],
    };

    html! {
        <>
        <form {onsubmit} class="w-full flex flex-col items-center mt-5">
            <h1 class="text-3xl font-bold text-slate-100">{"Add a new position"}</h1>
            <div class="flex flex-col rounded-2xl p-5 bg-slate-300">
                <PositionField hint="Position" />
                <input type="submit" class="btn-accent btn-hover" value={"Fetch Skills"}/>
            </div>
        </form>
        <div class="flex flex-col items-center mt-5">
            <h1 class="text-3xl font-bold text-slate-100">{"Select Skills"}</h1>
            <div class="flex flex-col max-w-sm rounded-2xl p-5 bg-slate-300">
                <SkillTable skills={dummy_data.skills} />
            </div>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
