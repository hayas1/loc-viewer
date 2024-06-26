use std::sync::Arc;

use tokei::{Language, Sort};
use yew::{prelude::*, suspense::use_future_with};
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};
use yew_router::hooks::{use_location, use_navigator, use_route};

use super::{
    background::{Pane, ResponsivePanesFrame},
    forms::{RepoInfoForms, RepoUrlBar},
    query_parameters::{QueryParams, StatisticsParamsModel, TableViewParamsModel},
    routes::{Route, RouterUnavailable, Unreachable},
};
use crate::{
    error::Result,
    github::{repository::GitHubRepository, statistics::Statistics},
};

pub const CAPTION: &str = "Statistics";

#[autoprops]
#[function_component(StatisticsPage)]
pub fn statistics_page(host: &String, owner: &String, repo: &String) -> HtmlResult {
    let Some(location) = use_location() else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let query = StatisticsParamsModel::from_query(&location.query::<Vec<(String, String)>>().unwrap());
    let repository = Arc::new(GitHubRepository::new(owner, repo));
    let repository_url = repository.to_url().unwrap().to_string();

    let fallback = html! {
        <div class={classes!("w-full", "h-full", "flex", "justify-center", "items-center")} aria-label="Loading">
            <div class={classes!("animate-spin", "inline-block", "w-8", "h-8", "border-4", "border-teal-600", "rounded-full", "border-t-transparent")}></div>
        </div>
    };
    Ok(html! {
        <ResponsivePanesFrame>
            <Pane class={classes!("p-6", "grow", "max-w-xs", "md:w-full", "md:max-w-full")}>
                <p>
                    <Icon icon_id={IconId::OcticonsMarkGithub16} class={classes!("mx-2", "inline-block")}/>
                    <a href={repository_url.clone()} class={classes!(
                        "border-b", "border-teal-500", "text-teal-500", "hover:text-teal-700",
                        "dark:border-teal-100", "dark:text-teal-100", "dark:hover:text-teal-200",
                    )}>
                        {repository_url}
                    </a>
                </p>
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
                    <div class={classes!("w-full")}>
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

    let result =
        use_future_with(repository.clone(), |_| async move { repository.get_statistics(&config).await.map(Arc::new) })?;

    Ok(html! {
        match &(*result) {
            Ok(statistics) => html! {
                <div class={classes!("pt-4")}>
                    <div class={classes!("pb-2", "inline-flex", "rounded-md", "text-sm", "hidden")} role="group"> // TODO implement other than table view
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
    fn order_by<O: Ord, F: Fn(&Language) -> O>(la: &Language, lb: &Language, f: F) -> std::cmp::Ordering {
        f(lb).cmp(&f(la))
    }

    let Some(location) = use_location() else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let query = TableViewParamsModel::from_query(&location.query::<Vec<(String, String)>>().unwrap());
    let mut languages: Vec<_> = statistics.languages.iter().collect();
    match query.map(|TableViewParamsModel { order_by }| order_by) {
        Ok(Some(Sort::Blanks)) => languages.sort_by(|(_, la), (_, lb)| order_by(la, lb, |l| l.blanks)),
        Ok(Some(Sort::Comments)) => languages.sort_by(|(_, la), (_, lb)| order_by(la, lb, |l| l.comments)),
        Ok(Some(Sort::Code)) => languages.sort_by(|(_, la), (_, lb)| order_by(la, lb, |l| l.code)),
        Ok(Some(Sort::Files)) => languages.sort_by(|(_, la), (_, lb)| order_by(la, lb, |l| l.reports.len())),
        Ok(Some(Sort::Lines)) => languages.sort_by(|(_, la), (_, lb)| order_by(la, lb, |l| l.lines())),
        Ok(None) => (),                                          // TODO ascending language name
        Err(_) => gloo::console::warn!("Failed to parse query"), // TODO error handling
    }

    let leftmost = classes!("sticky", "left-0", "z-[50]"); // TODO long name language, z-index for scroll
    let table_header = classes!("text-teal-900", "bg-teal-50", "dark:text-teal-50", "dark:bg-teal-800");
    let (lm, th) = (leftmost.clone(), table_header.clone());

    let col: [(_, _, _, Box<dyn Fn(&tokei::Language) -> _>); 6] = [
        ("Language", None, IconId::OcticonsRocket16, Box::new(|l| l.reports.len())),
        ("Files", Some(Sort::Files), IconId::OcticonsFile16, Box::new(|l| l.reports.len())),
        ("Lines", Some(Sort::Lines), IconId::OcticonsThreeBars16, Box::new(|l| l.lines())),
        ("Code", Some(Sort::Code), IconId::OcticonsCode16, Box::new(|l| l.code)),
        ("Comments", Some(Sort::Comments), IconId::OcticonsComment16, Box::new(|l| l.comments)),
        ("Blanks", Some(Sort::Blanks), IconId::OcticonsDash16, Box::new(|l| l.blanks)),
    ];

    let focused = use_state(|| None);

    Ok(html! {
        <table class={classes!("table-auto")}>
            <thead>
                <tr>
                    {for col.iter().enumerate().map(|(j, (title, sort, icon_id, _))| {
                        html! {
                            if j == 0 {
                                <th scope="col" class={classes!(lm.clone(), th.clone())} title={&title[..]}>
                                    <TableHeader>
                                        <Icon icon_id={icon_id.clone()}/>
                                    </TableHeader>
                                </th>
                            } else {
                                <th scope="col" class={classes!(th.clone())} title={&title[..]}>
                                    <TableHeaderCol focused={*focused} col={j} sort={sort.clone()} title={&title[..]}>
                                        <Icon icon_id={icon_id.clone()}/>
                                    </TableHeaderCol>
                                </th>
                            }
                        }
                    })}
                </tr>
            </thead>
            <tbody>
                {for languages.iter().enumerate().map(|(i, (language_type, language))| {
                    html! {
                        <tr class={classes!()}>
                            {for col.iter().enumerate().map(|(j, (_, _, _, f))| {
                                html! {
                                    if j == 0 {
                                        <th scope="row" class={classes!(lm.clone())}>
                                            <TableHeaderRow class={classes!(th.clone())} focused={*focused} row={i} title={language_type.to_string()}>
                                                { language_type.to_string() }
                                            </TableHeaderRow>
                                        </th>
                                    } else {
                                        <td>
                                            <TableCell focused={focused.clone()} pos={(i, j)} class={classes!("text-right")}>
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
#[function_component(TableHeader)]
pub fn table_header_row(children: &Children) -> HtmlResult {
    let (Some(navigator), Some(location), Some(route)) = (use_navigator(), use_location(), use_route::<Route>()) else {
        return Ok(html! { <RouterUnavailable/> });
    };

    let clear_order = Callback::from(move |_| {
        let param: Result<Vec<(String, String)>> = (|| {
            let statistics_params = StatisticsParamsModel::from_query(
                &location.query::<Vec<(String, String)>>().map_err(anyhow::Error::from)?,
            )?;
            let params = [statistics_params.into_query()?];
            Ok(params.into_iter().flatten().collect())
        })();
        match param {
            Ok(param) => match navigator.replace_with_query(&route, &param) {
                Ok(_) => (),
                Err(err) => gloo::console::error!(err.to_string()), // TODO error handling
            },
            Err(err) => gloo::console::error!(err.to_string()), // TODO error handling
        }
    });

    Ok(html! {
        <div class={classes!("flex", "justify-center")}>
            <button onclick={clear_order}>
                { children.clone() }
            </button>
        </div>
    })
}

#[autoprops]
#[function_component(TableHeaderCol)]
pub fn table_header_col(
    focused: &Option<(usize, usize)>,
    col: usize,
    title: &String,
    #[prop_or_default] sort: &Option<Sort>,
    #[prop_or_default] class: &Classes,
    children: &Children,
) -> HtmlResult {
    // TODO implement sort button
    let (Some(navigator), Some(location), Some(route)) = (use_navigator(), use_location(), use_route::<Route>()) else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let Ok(statistics_params): Result<StatisticsParamsModel> = (|| {
        Ok(StatisticsParamsModel::from_query(&location.query::<Vec<(String, String)>>().map_err(anyhow::Error::from)?)?)
    })() else {
        return Ok(html! { <Unreachable/> });
    };
    let Ok(table_params): Result<TableViewParamsModel> = (|| {
        Ok(TableViewParamsModel::from_query(&location.query::<Vec<(String, String)>>().map_err(anyhow::Error::from)?)?)
    })() else {
        return Ok(html! { <Unreachable/> });
    };

    let popup = matches!(focused.map(|(_, c)| c == col), Some(true));
    let order_by_this = &table_params.order_by == sort;

    let order_by = {
        let (navigator, route) = (navigator.clone(), route.clone());
        let (statistics_params, table_params) = (statistics_params, table_params);
        let sort = sort.clone();
        Callback::from(move |_| {
            let param: Result<Vec<(String, String)>> = (|| {
                let table_params = TableViewParamsModel { order_by: sort, ..table_params };
                let params = [statistics_params.into_query()?, table_params.into_query()?];
                Ok(params.into_iter().flatten().collect())
            })();
            match param {
                Ok(param) => match navigator.replace_with_query(&route, &param) {
                    Ok(_) => (),
                    Err(err) => gloo::console::error!(err.to_string()), // TODO error handling
                },
                Err(err) => gloo::console::error!(err.to_string()), // TODO error handling
            }
        })
    };

    Ok(html! {
        <div class={classes!("flex", "justify-center", "relative", "bg-cover", class.clone())} title={title.clone()}>
            <button onclick={order_by} class={classes!("px-4", "py-2", popup.then(|| "opacity-20"), order_by_this.then(|| classes!("bg-teal-200", "dark:bg-teal-700")))}>
                {children.clone()}
            </button>
            if popup {
                <div class={classes!("absolute", "top-1/2", "left-1/2", "-translate-x-1/2", "-translate-y-1/2")}>
                    {title.clone()}
                </div>
            }
        </div>
    })
}

#[autoprops]
#[function_component(TableHeaderRow)]
pub fn table_header_row(
    focused: &Option<(usize, usize)>,
    row: usize,
    title: &String,
    #[prop_or_default] class: &Classes,
    children: &Children,
) -> HtmlResult {
    let popup = matches!(focused.map(|(r, _)| r == row), Some(true));

    Ok(html! {
        <p class={classes!("p-2", popup.then(|| classes!("bg-teal-100", "dark:bg-teal-700")), class.clone())}
            title={title.clone()}>
            {children.clone()}
        </p>
    })
}

#[autoprops]
#[function_component(TableCell)]
pub fn table_cell(
    focused: &UseStateHandle<Option<(usize, usize)>>,
    pos: (usize, usize),
    #[prop_or_default] class: &Classes,
    children: &Children,
) -> HtmlResult {
    let highlight =
        focused.map(|(i, j)| i == pos.0 || j == pos.1).map(|b| b.then(|| classes!("bg-teal-50", "dark:bg-teal-800")));

    let focus = {
        let (focused, pos) = (focused.clone(), pos.clone());
        Callback::from(move |_| focused.set(Some(pos.clone())))
    };
    let blur = {
        let focused = focused.clone();
        Callback::from(move |_| focused.set(None))
    };

    Ok(html! {
        <div onmouseenter={focus} onmouseleave={blur} class={classes!("px-4", "py-2", class.clone(), highlight)}>
            { children.clone() }
        </div>
    })
}
