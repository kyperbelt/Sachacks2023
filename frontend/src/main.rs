use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsValue;
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
    html! {
        <table class="table-auto">
            <thead>
                <tr>
                    <th class="px-4 py-2">{"Skills"}</th>
                    <th class="px-4 py-2">{"Did"}</th>
                </tr>
            </thead>
            <tbody>
                {for props.skills.iter().map(|skill| html! {
                    <tr>
                        <td class="border px-4 py-2">{skill.0.clone()}</td>
                        <td class="border px-4 py-2">
                            <input type="checkbox" class="form-checkbox h-5 w-5 text-slate-600" checked={skill.1}/>
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
