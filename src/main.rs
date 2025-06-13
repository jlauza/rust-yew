use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };


    html! {
        <>
            <h1>{ "Counter Example" }</h1>
            <button {onclick}>{ "Add +1" }</button>
            <p>{format!("Counter value: {}", *counter)}</p>
        </>
    }    
}

fn main() {
    yew::Renderer::<App>::new().render();
}