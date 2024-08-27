use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub fn AppContainer(children: &Html) -> Html {
    html!{
        <div id="app" class="container h-screen mx-auto p-1">
            {children.clone()}
        </div>
    }
}
