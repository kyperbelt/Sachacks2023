use yew::prelude::*;

#[function_component]
fn App() -> Html {

    html! {
        <button class="btn btn-accent btn-hover">{"Hello world"}</button> 
    }

}

fn main() {
    let x = 0;
    println!("{}", x);
    yew::Renderer::<App>::new().render();
}
