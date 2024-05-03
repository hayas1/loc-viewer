use yew::prelude::*;
use yew_router::prelude::*;

use super::{home::Home, navbar::Navbar, statistics::Statistics};

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
pub fn main() -> HtmlResult {
    Ok(html! {
        <BrowserRouter basename="/loc-viewer/"> // TODO do not hard code basename
            <Navbar/>
            <Switch<Route> render={Route::switch} />
        </BrowserRouter>
    })
}

#[function_component(NotFound)]
pub fn not_found() -> HtmlResult {
    let Some(navigator) = use_navigator() else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let on_click = Callback::from(move |_| navigator.push(&Route::Home));
    Ok(html! {
        <div>
            <h1>{ "404" }</h1>
            <button onclick={on_click}>{ "Go Home" }</button>
        </div>
    })
}

#[function_component(RouterUnavailable)]
pub fn router_unavailable() -> HtmlResult {
    Ok(html! {
        <div>
            <h1>{ "Router Unavailable" }</h1>
        </div>
    })
}
