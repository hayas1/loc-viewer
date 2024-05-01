pub mod routes;
pub mod statistics;

use std::sync::Arc;

use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::github::repository::GitHubRepository;
use statistics::Table;

#[function_component(App)]
pub fn app() -> HtmlResult {
    let repository_input = use_node_ref();
    let input_handle = use_state(|| "https://github.com/hayas1/loc-viewer".to_string());
    let repository = Arc::new(GitHubRepository::from_url(&Url::parse(&*input_handle.clone()).unwrap()).unwrap());

    let on_change = {
        let repository_input = repository_input.clone();
        let input_handle = input_handle.clone();
        Callback::from(move |_| {
            if let Some(input) = repository_input.cast::<HtmlInputElement>() {
                input_handle.set(input.value());
            }
        })
    };

    let input_id = "repository-input";
    Ok(html! {
        <div>
            // <form class="w-full">
                <div class="md:flex md:items-center mb-6">
                    <div class="">
                        <label for={input_id} class="block text-gray-700 text-sm font-bold mb-2">
                            { "Repository:" }
                        </label>
                    </div>
                    <div class="w-full">
                        <input ref={repository_input}
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            onchange={on_change}
                            id={input_id}
                            type="text"
                            value={(*input_handle).clone()}
                        />
                    </div>
                </div>
            // </form>
            <div class="w-full">
                <Table repository={repository} />
            </div>
        </div>
    })
}
