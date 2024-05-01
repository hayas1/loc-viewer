use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Home => html! { <crate::components::App/> },
            Route::NotFound => html! { <NotFound/> },
        }
    }
}

#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <BrowserRouter basename="/loc-viewer/"> // TODO do not hard code basename
            <Switch<Route> render={Route::switch} />
        </BrowserRouter>
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();
    let on_click = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "404" }</h1>
            <button onclick={on_click}>{ "Go Home" }</button>
        </div>
    }
}
