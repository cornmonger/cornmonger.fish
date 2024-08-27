use yew::prelude::*;

pub(crate) fn run() {
    yew::Renderer::<App>::new().render();
}

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
        <div>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded-full" {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
