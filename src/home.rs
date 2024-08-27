use yew::prelude::*;
use yew_autoprops::autoprops;

pub(crate) fn run() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <AppContainer>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded-full">{ "+1" }</button>
        </AppContainer>
    }
}

#[autoprops]
#[function_component]
pub fn AppContainer(children: &Html) -> Html {
    html!{
        <div id="app" class="container h-screen mx-auto p-1">
            {children.clone()}
        </div>
    }
}