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
    pub position: String,
}

#[function_component]
fn App() -> Html {
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let greeting = String::from("Hi there");
        web_sys::console::log_1(&greeting.into()); // if uncommented will print
        let body = reqwest::blocking::post("http://localhost/test")
            .unwrap()
            .json(serde_json::to_string(&PositionPayload {
                position: "test".to_string(),
            }))
            .unwrap();
    });

    html! {
        <>
        <form {onsubmit} class="w-full flex flex-col items-center mt-5">
            <h1 class="text-3xl font-bold text-slate-100">{"Add a new position"}</h1>
            <div class="flex flex-col max-w-sm rounded-2xl p-5 bg-slate-300">
                <PositionField hint="Position" />
                <input type="submit" class="btn-accent btn-hover" value={"Submit"}/>
            </div>
        </form>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
