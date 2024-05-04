use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

use super::{home::Home, navbar::Navbar, statistics::Statistics, BASENAME};

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

#[autoprops]
#[function_component(Main)]
pub fn main() -> HtmlResult {
    Ok(html! {
        <BrowserRouter basename={BASENAME}>
            <Navbar/>
            <Switch<Route> render={Route::switch} />
        </BrowserRouter>
    })
}

#[autoprops]
#[function_component(GoHome)]
pub fn go_home(navigator: &Option<Navigator>, html: &Html) -> HtmlResult {
    Ok(html! {
        if let Some(nav) = navigator.clone() {
            <button type="none" onclick={Callback::from(move |_| nav.push(&Route::Home))}>{ html.clone() }</button>
        } else {
            <a href={BASENAME}>{ html.clone() }</a>
        }
    })
}

#[autoprops]
#[function_component(NotFound)]
pub fn not_found() -> HtmlResult {
    let navigator = use_navigator();
    Ok(html! {
        <div>
            <h1>{ "404" }</h1>
            <GoHome navigator={navigator} html={ "Go Home" }/>
        </div>
    })
}

#[autoprops]
#[function_component(RouterUnavailable)]
pub fn router_unavailable() -> HtmlResult {
    Ok(html! {
        <div>
            <h1>{ "Router Unavailable" }</h1>
            <GoHome navigator={None} html={ "Go Home" }/>
        </div>
    })
}
