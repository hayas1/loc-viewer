use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

use super::{
    background::{Background, Navbar, Screen},
    darkmode::Theme,
    home::HomePage,
    statistics::StatisticsPage,
    BASENAME,
};

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
            Route::Home => html! { <HomePage/> },
            Route::Statistics { host, owner, repo } => html! { <StatisticsPage {host} {owner} {repo}/> },
            Route::NotFound => html! { <NotFound/> },
        }
    }
}

#[autoprops]
#[function_component(Main)]
pub fn main() -> HtmlResult {
    let ctx = use_reducer(|| Theme::get()); // TODO struct Context
    Ok(html! {
        <BrowserRouter basename={BASENAME}>
            <ContextProvider<UseReducerHandle<Theme>> context={ctx}>
                <Screen>
                    <Navbar/>
                    <Background>
                        <Switch<Route> render={Route::switch} />
                    </Background>
                </Screen>
            </ContextProvider<UseReducerHandle<Theme>>>
        </BrowserRouter>
    })
}

#[autoprops]
#[function_component(GoHome)]
pub fn go_home(navigator: &Option<Navigator>, children: &Children) -> HtmlResult {
    Ok(html! {
        if let Some(nav) = navigator.clone() {
            <button type="none" onclick={Callback::from(move |_| nav.push(&Route::Home))}>{ children.clone() }</button>
        } else {
            <a href={BASENAME}>{ children.clone() }</a>
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
            <GoHome navigator={navigator}>
                <span>{ "Go Home" }</span>
            </GoHome>
        </div>
    })
}

#[autoprops]
#[function_component(RouterUnavailable)]
pub fn router_unavailable() -> HtmlResult {
    Ok(html! {
        <div>
            <h1>{ "Router Unavailable" }</h1>
            <GoHome navigator={None}>
                <span>{ "Go Home" }</span>
            </GoHome>
        </div>
    })
}

#[autoprops]
#[function_component(InvalidContext)]
pub fn invalid_context() -> HtmlResult {
    Ok(html! {
        <div>
            <h1>{ "Invalid Context" }</h1>
            <GoHome navigator={None}>
                <span>{ "Go Home" }</span>
            </GoHome>
        </div>
    })
}

#[autoprops]
#[function_component(Unreachable)]
pub fn unreachable() -> HtmlResult {
    Ok(html! {
        <div>
            <h1>{ "Unreachable" }</h1>
            <GoHome navigator={None}>
                <span>{ "Go Home" }</span>
            </GoHome>
        </div>
    })
}
