pub mod routes;

use std::sync::Arc;

use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::github::repository::GitHubRepository;

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

#[autoprops]
#[function_component(Table)]
pub fn table(repository: &Arc<GitHubRepository>) -> HtmlResult {
    let repository = repository.clone();
    let result = use_state(|| None);
    {
        let (result, repository) = (result.clone(), repository.clone());
        use_effect_with(repository.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let statistics = repository.get_statistics(&Default::default()).await;
                result.set(Some(statistics));
            })
        });
    }

    Ok(html! {
        match &(*result) {
            Some(Ok(statistics)) => html! {
                <table class="table-auto">
                    <thead>
                        <tr>
                            <th>{ "Language" }</th>
                            <th>{ "Files" }</th>
                            <th>{ "Lines" }</th>
                            <th>{ "Code" }</th>
                            <th>{ "Comments" }</th>
                            <th>{ "Blanks" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for statistics.languages.iter().map(|(language_type, language)| {
                                html! {
                                    <tr>
                                        <td>{ language_type.to_string() }</td>
                                        <td>{ language.reports.len() }</td>
                                        <td>{ language.lines() }</td>
                                        <td>{ language.code }</td>
                                        <td>{ language.comments }</td>
                                        <td>{ language.blanks }</td>
                                        // <td>{ language.total() }</td>
                                   </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
            },
            Some(Err(err)) => html! { format!("error occurred: {err:?}") },
            None => html! { format!("loading...") },
        }
    })
}
