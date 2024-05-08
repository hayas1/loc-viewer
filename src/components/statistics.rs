use std::sync::Arc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::hooks::use_location;

use super::{
    background::{Pane, ResponsivePanesFrame},
    forms::{RepoInfoForms, RepoUrlBar},
    query_parameters::ParamsModel,
    routes::RouterUnavailable,
};
use crate::{
    error::{render::Unreachable, Result},
    github::{repository::GitHubRepository, statistics::Statistics},
};

#[autoprops]
#[function_component(StatisticsPage)]
pub fn statistics_page(host: &String, owner: &String, repo: &String) -> HtmlResult {
    let Some(location) = use_location() else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let query = ParamsModel::from_query(&location.query::<Vec<(String, String)>>().unwrap());
    let repository = Arc::new(GitHubRepository::new(owner, repo));
    Ok(html! {
        <ResponsivePanesFrame>
            <Pane class={classes!("p-6", "grow", "w-full")}>
                <TableResult repository={repository}/>
            </Pane>
            <Pane class={classes!("p-6", "w-max", "max-w-xs", "flex", "flex-col", "justify-start")}>
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
#[function_component(TableResult)]
pub fn table_result(repository: &Arc<GitHubRepository>) -> HtmlResult {
    let repository = repository.clone();
    let result = use_state(|| None);
    {
        let (result, repository) = (result.clone(), repository.clone());
        use_effect_with(repository.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let statistics = repository.get_statistics(&Default::default()).await;
                let arc = statistics.map(Arc::new);
                result.set(Some(arc));
            })
        });
    }

    Ok(html! {
        match &(*result) {
            Some(Ok(statistics)) => html! {
                <div class={classes!("flex", "overflow-x-auto")}>
                    <div class={classes!("flex-none", "w-10")}>
                        <TableView statistics={statistics.clone()}/>
                    </div>
                </div>
            },
            Some(Err(err)) => html! { format!("error occurred: {err:?}") },
            None => html! { format!("loading...") },
        }
    })
}

#[autoprops]
#[function_component(TableView)]
pub fn table_view(statistics: &Arc<Statistics>) -> HtmlResult {
    let caption_input = use_node_ref();
    let cell_pudding = classes!("px-4", "py-2");
    let leftmost = classes!("py-2", "sticky", "left-0"); // TODO long name language
    let table_header = classes!("text-teal-900", "bg-teal-50", "dark:text-teal-50", "dark:bg-teal-800");

    let (cp, lm, th) = (cell_pudding.clone(), leftmost.clone(), table_header.clone());
    Ok(html! {
        <table class={classes!("table-auto")}>
            <caption class={classes!("caption-top", "text-left")}>
                <input ref={caption_input.clone()}
                    disabled=true
                    class={classes!("px-4", "appearance-none", "bg-transparent", "sticky", "left-0")}
                    value="Statistics"
                />
            </caption>
            <thead>
                <tr>
                    <th scope="col" class={classes!(lm.clone(), th.clone())}>{ "Language" }</th>
                    <th scope="col" class={classes!(cp.clone(), th.clone())}>{ "Files" }</th>
                    <th scope="col" class={classes!(cp.clone(), th.clone())}>{ "Lines" }</th>
                    <th scope="col" class={classes!(cp.clone(), th.clone())}>{ "Code" }</th>
                    <th scope="col" class={classes!(cp.clone(), th.clone())}>{ "Comments" }</th>
                    <th scope="col" class={classes!(cp.clone(), th.clone())}>{ "Blanks" }</th>
                </tr>
            </thead>
            <tbody>
                {
                    for statistics.languages.iter().map(|(language_type, language)| {
                        html! {
                            <tr class={classes!("border-b")}>
                                <th scope="row" class={classes!(lm.clone(), th.clone())}>{ language_type.to_string() }</th>
                                <td class={classes!(cp.clone())}>
                                    <TableCell caption_input={caption_input.clone()} language={language_type.to_string()} category="Files">
                                        { language.reports.len() }
                                    </TableCell>
                                </td>
                                <td class={classes!(cp.clone())}>
                                    <TableCell caption_input={caption_input.clone()} language={language_type.to_string()} category="Lines">
                                        { language.lines() }
                                    </TableCell>
                                </td>
                                <td class={classes!(cp.clone())}>
                                    <TableCell caption_input={caption_input.clone()} language={language_type.to_string()} category="Code">
                                        { language.code }
                                    </TableCell>
                                </td>
                                <td class={classes!(cp.clone())}>
                                    <TableCell caption_input={caption_input.clone()} language={language_type.to_string()} category="Comments">
                                        { language.comments }
                                    </TableCell>
                                </td>
                                <td class={classes!(cp.clone())}>
                                    <TableCell caption_input={caption_input.clone()} language={language_type.to_string()} category="Blanks">
                                        { language.blanks }
                                    </TableCell>
                                </td>
                                // <td>{ language.total() }</td>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    })
}

#[autoprops]
#[function_component(TableCell)]
pub fn table_cell(caption_input: &NodeRef, language: &String, category: &String, children: &Children) -> HtmlResult {
    let overwrite_caption = {
        let caption_input = caption_input.clone();
        let (language, category) = (language.clone(), category.clone());
        Callback::from(move |_| {
            let result: Result<_> = (|| {
                let caption = caption_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .set_value(&format!("{} {}", language.clone(), category.clone()));
                Ok(caption)
            })();
            match result {
                Ok(_) => {}
                Err(err) => gloo::console::error!("error:", err.to_string()),
            }
        })
    };

    Ok(html! {
        <button onclick={overwrite_caption}>
            { children.clone() }
        </button>
    })
}
