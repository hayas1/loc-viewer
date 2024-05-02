use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use super::routes::Route;
use crate::{
    error::{render::Unreachable, Result},
    github::repository::GitHubRepository,
};

#[function_component(Home)]
pub fn home() -> HtmlResult {
    let example = "https://github.com/hayas1/loc-viewer";
    let repository_input = use_node_ref();

    let navigator = use_navigator().unwrap();
    let on_click = {
        let repository_input = repository_input.clone();
        Callback::from(move |_| {
            let route: Result<Route> = (|| {
                let input = repository_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?;
                let value = input.value();
                let repository = if value.is_empty() {
                    GitHubRepository::from_url(&Url::parse(example).map_err(anyhow::Error::from)?)?
                } else {
                    GitHubRepository::from_url(&Url::parse(&value).map_err(anyhow::Error::from)?)?
                };
                let (host, GitHubRepository { owner, repo }) = (repository.host(), repository);
                Ok(Route::Statistics { host, owner, repo })
            })();
            match route {
                Ok(route) => navigator.push(&route),
                Err(err) => gloo::console::error!(err.to_string()), // TODO error handling
            }
        })
    };

    Ok(html! {
        <div class="w-full max-w-sm">
            <div class="flex items-center border-b border-teal-500 py-2">
                <input ref={repository_input}
                    class="appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none"
                    type="text"
                    placeholder={example}
                    aria-label="Repository"/>
                <button
                    class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded-lg"
                    onclick={on_click}
                    type="button"
                >
                    { "Toukei" }
                </button>
          </div>
        </div>
    })
}
