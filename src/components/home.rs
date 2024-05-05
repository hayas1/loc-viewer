use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

use super::forms::{RepoInfoForms, RepoUrlBar};
use crate::{
    error::{render::Unreachable, Result},
    github::repository::GitHubRepository,
};

#[autoprops]
#[function_component(Home)]
pub fn home() -> HtmlResult {
    let h1 = "Statistics about the code";
    let description = "Enter the repository URL or information, and view statistics about the code.";
    Ok(html! {
        <div class={classes!("p-2", "flex", "justify-center")}>
            <div class={classes!("p-6", "container", "rounded-xl", "max-w-screen-lg", "bg-white", "dark:text-teal-50", "dark:bg-teal-900")}>
                <div class={classes!("flex", "items-center")}>
                    <Icon icon_id={IconId::HeroiconsOutlineClock} class={classes!("inline-block", "h-6")}/>
                    <h1 class={classes!("p-2", "text-teal-900", "dark:text-teal-50", "text-xl")}>
                        { h1 }
                    </h1>
                </div>
                <p class={classes!("p-2", "text-slate-500", "dark:text-slate-400", "text-sm")}>
                    { description }
                </p>
                <div class={classes!("md:flex", "md:justify-center", "md:items-center", "gap-4")}>
                    <RepoUrlBar/>
                    <p class={classes!("p-2", "text-teal-900", "dark:text-teal-50", "text-center")}>{"or"}</p>
                    <RepoInfoForms/>
                </div>
            </div>
        </div>
    })
}
