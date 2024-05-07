use std::sync::Arc;

use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::hooks::use_location;

use super::{
    background::{Pane, ResponsivePanesFrame},
    forms::{RepoInfoForms, RepoUrlBar},
    query_parameters::ParamsModel,
    routes::RouterUnavailable,
};
use crate::github::repository::GitHubRepository;

#[autoprops]
#[function_component(Statistics)]
pub fn statistics(host: &String, owner: &String, repo: &String) -> HtmlResult {
    let Some(location) = use_location() else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let query = ParamsModel::from_query(&location.query::<Vec<(String, String)>>().unwrap());
    let repository = Arc::new(GitHubRepository::new(owner, repo));
    Ok(html! {
        <ResponsivePanesFrame>
            <Pane class={classes!("p-6", "grow", "w-full")}>
                <p>{format!("{query:?}")}</p>
                <div class="flex overflow-x-auto">
                    <div class="flex-none w-10">
                        <Table repository={repository}/>
                    </div>
                </div>
            </Pane>
            <Pane class={classes!("p-6", "max-w-sm", "flex", "flex-col", "justify-start")}>
                <div class={classes!("h-full", "w-full", "flex", "flex-col", "justify-start")}>
                    <label for="repository-url" class={classes!("w-full")}>{"URL"}</label>
                    <div class={classes!("w-full", "pt-4")}>
                        <RepoUrlBar id="repository-url"/>
                    </div>
                </div>
                <p class={classes!("p-2", "text-teal-900", "dark:text-teal-50", "text-center")}>{"or"}</p>
                <div class={classes!("flex", "flex-col", "justify-start")}>
                    <label for="repository-info">{"Information"}</label>
                    <div class={classes!("w-full", "pt-4")}>
                        <RepoInfoForms/>
                    </div>
                </div>
            </Pane>
        </ResponsivePanesFrame>
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
