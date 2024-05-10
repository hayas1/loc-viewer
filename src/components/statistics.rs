use std::sync::Arc;

use web_sys::HtmlInputElement;
use yew::{prelude::*, suspense::use_future};
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};
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

pub const CAPTION: &str = "Statistics";

#[autoprops]
#[function_component(StatisticsPage)]
pub fn statistics_page(host: &String, owner: &String, repo: &String) -> HtmlResult {
    let Some(location) = use_location() else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let query = ParamsModel::from_query(&location.query::<Vec<(String, String)>>().unwrap());
    let repository = Arc::new(GitHubRepository::new(owner, repo));

    let fallback = html! {
        <div class={classes!("w-full", "h-full", "flex", "justify-center", "items-center")} aria-label="Loading">
            <div class={classes!("animate-spin", "inline-block", "w-8", "h-8", "border-4", "border-teal-600", "rounded-full", "border-t-transparent")}></div>
        </div>
    };
    Ok(html! {
        <ResponsivePanesFrame>
            <Pane class={classes!("p-6", "grow", "w-full")}>
                <Suspense {fallback}>
                    <StatisticsView repository={repository}/>
                </Suspense>
            </Pane>
            <Pane class={classes!("p-6", "max-w-xs", "flex", "flex-col", "justify-start")}>
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
#[function_component(StatisticsView)]
pub fn statistics_view(repository: &Arc<GitHubRepository>) -> HtmlResult {
    let repository = repository.clone();
    let config = Default::default();

    let result = use_future(|| async move { repository.get_statistics(&config).await.map(Arc::new) })?;

    Ok(html! {
        match &(*result) {
            Ok(statistics) => html! {
                <div>
                    <div class={classes!("pb-4", "inline-flex", "rounded-md", "text-sm")} role="group">
                        <button type="button"
                            class={classes!("px-2", "border", "rounded-s-full", "hover:bg-teal-50", "hover:dark:bg-teal-800", "focus:ring-2")}
                        >
                            <p>{"Table"}</p>
                        </button>
                        <button type="button"
                            class={classes!("px-2", "border-t", "border-b", "hover:bg-teal-50", "hover:dark:bg-teal-800", "focus:ring-2")}
                        >
                            <p class="text-teal-900/50 dark:text-teal-50/50" title="unimplemented">{"Heatmap"}</p>
                        </button>
                        <button type="button"
                            class={classes!("px-2", "border", "rounded-e-full", "hover:bg-teal-50", "hover:dark:bg-teal-800", "focus:ring-2")}
                        >
                            <p class="text-teal-900/50 dark:text-teal-50/50" title="unimplemented">{"Chart"}</p>
                        </button>
                    </div>
                    <div class={classes!("flex", "overflow-x-auto")}>
                        <div class={classes!("flex-none", "w-10")}>
                            <TableView statistics={statistics.clone()}/>
                        </div>
                    </div>
                </div>
            },
            Err(err) => html! { format!("error occurred: {err:?}") },
        }
    })
}

#[autoprops]
#[function_component(TableView)]
pub fn table_view(statistics: &Arc<Statistics>) -> HtmlResult {
    let caption_input = use_node_ref();
    let cell_pudding = classes!("px-4", "py-2");
    let leftmost = classes!("p-2", "sticky", "left-0"); // TODO long name language
    let table_header = classes!("text-teal-900", "bg-teal-50", "dark:text-teal-50", "dark:bg-teal-800");
    let (cp, lm, th) = (cell_pudding.clone(), leftmost.clone(), table_header.clone());

    let col: [(_, _, Box<dyn Fn(&tokei::Language) -> _>); 6] = [
        ("Language", IconId::OcticonsRocket16, Box::new(|l| l.reports.len())),
        ("Files", IconId::OcticonsFile16, Box::new(|l| l.reports.len())),
        ("Lines", IconId::OcticonsThreeBars16, Box::new(|l| l.lines())),
        ("Code", IconId::OcticonsCode16, Box::new(|l| l.code)),
        ("Comments", IconId::OcticonsComment16, Box::new(|l| l.comments)),
        ("Blanks", IconId::OcticonsDash16, Box::new(|l| l.blanks)),
    ];

    Ok(html! {
        <table class={classes!("table-auto")}>
            <caption class={classes!("caption-top", "text-left")}>
                <input ref={caption_input.clone()}
                    disabled=true
                    class={classes!("px-4", "appearance-none", "bg-transparent", "sticky", "left-0")}
                    value={CAPTION}
                />
            </caption>
            <thead>
                <tr>
                    {for col.iter().enumerate().map(|(i, (name, icon_id, _))| {
                        html! {
                            if i == 0 {
                                <th scope="col" class={classes!(lm.clone(), th.clone())} title={&name[..]}>
                                    <div class={classes!("flex", "justify-center")}>
                                        <Icon icon_id={icon_id.clone()}/>
                                    </div>
                                </th>
                            } else {
                                <th scope="col" class={classes!(cp.clone(), th.clone())} title={&name[..]}>
                                    <Icon icon_id={icon_id.clone()}/>
                                </th>
                            }
                        }
                    })}
                </tr>
            </thead>
            <tbody>
                {for statistics.languages.iter().map(|(language_type, language)| {
                        html! {
                            <tr class={classes!("border-b")}>
                                {for col.iter().enumerate() .map(|(i, (name, _, f))| {
                                    html! {
                                        if i == 0 {
                                            <th scope="row" class={classes!(lm.clone(), th.clone())}>
                                                { language_type.to_string() }
                                            </th>
                                        } else {
                                            <td class={classes!(cp.clone())}>
                                                <TableCell caption_input={caption_input.clone()} language={language_type.to_string()} category={&name[..]}>
                                                    { f(language) }
                                                </TableCell>
                                            </td>
                                        }
                                    }
                                })}
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
    let focused = use_state(|| false);
    let focus = {
        let (caption_input, focused) = (caption_input.clone(), focused.clone());
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
                Ok(_) => focused.set(true),
                Err(err) => gloo::console::error!("error:", err.to_string()),
            }
        })
    };
    let blur = {
        let (caption_input, focused) = (caption_input.clone(), focused.clone());
        Callback::from(move |_| {
            let result: Result<_> = (|| {
                let caption = caption_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .set_value(CAPTION);
                Ok(caption)
            })();
            match result {
                Ok(_) => focused.set(false),
                Err(err) => gloo::console::error!("error:", err.to_string()),
            }
        })
    };

    let focused_classes = classes!(focused.then(|| classes!(
        "rounded-xl",
        "rounded-full",
        "text-teal-50",
        "bg-teal-800",
        "dark:text-teal-50",
        "dark:bg-teal-700"
    )));
    Ok(html! {
        <div>
            <button onclick={focus} class={focused_classes.clone()}>
                { children.clone() }
            </button>
            if *focused {
                // TODO do not use modal like closing mechanism ?
                <div onclick={blur} class={classes!("flex", "justify-end", "absolute",
                    "top-0", "left-0", "w-full", "h-full", "min-w-screen", "min-h-screen"
                )}>
                </div>
            }
        </div>
    })
}
