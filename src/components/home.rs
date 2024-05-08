use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};

use super::{
    background::{Pane, ResponsivePanesFrame},
    forms::{RepoInfoForms, RepoUrlBar},
};

#[autoprops]
#[function_component(HomePage)]
pub fn home_page() -> HtmlResult {
    let h1 = "Statistics about the code";
    let description = "Enter the repository URL or information, and view statistics about the code.";
    Ok(html! {
        <ResponsivePanesFrame>
            <Pane class={classes!("p-6", "w-full")}>
                <div class={classes!("flex", "items-center")}>
                    <Icon icon_id={IconId::HeroiconsOutlineClock} class={classes!("inline-block", "h-6")}/>
                    <h1 class={classes!("p-2", "text-teal-900", "dark:text-teal-50", "text-xl")}>
                        { h1 }
                    </h1>
                </div>
                <p class={classes!("p-2", "text-slate-500", "dark:text-slate-400", "text-sm")}>
                    { description }
                </p>
                <div class={classes!("pt-4", "md:flex", "md:justify-center", "md:items-start", "gap-4")}>
                    <div class={classes!("h-full", "w-full", "flex", "flex-col", "justify-start")}>
                        <label for="repository-url" class={classes!("w-full")}>{"URL"}</label>
                        <div class={classes!("w-full", "pt-4")}>
                            <RepoUrlBar id="repository-url"/>
                        </div>
                    </div>
                    <div class={classes!("md:flex", "md:justify-center", "md:items-center", "gap-4")}>
                        <p class={classes!("p-2", "text-teal-900", "dark:text-teal-50", "text-center")}>{"or"}</p>
                        <div class={classes!("max-w-screen-md")}>
                            <label for="repository-info">{"Information"}</label>
                            <RepoInfoForms/>
                        </div>
                    </div>
                </div>
            </Pane>
        </ResponsivePanesFrame>
    })
}
