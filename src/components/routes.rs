use yew::prelude::*;
use yew_router::prelude::*;

use super::{home::Home, statistics::Statistics};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/toukei/:host/:owner/:repo")]
    Statistics { host: String, owner: String, repo: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Home => html! { <Home/> },
            Route::Statistics { host, owner, repo } => html! { <Statistics {host} {owner} {repo}/> },
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
