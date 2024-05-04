use once_cell::sync::Lazy;
use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

use crate::{
    error::{render::Unreachable, Result},
    github::repository::GitHubRepository,
};

use super::{REPOSITORY, routes::{GoHome, Route}};

pub const NAVBAR_INPUT_CLASSES: Lazy<Classes> = Lazy::new(|| {
    classes!(
        "appearance-none",
        "bg-teal-50",
        "dark:bg-teal-800",
        "border",
        "border-teal-700",
        "text-teal-900",
        "dark:text-teal-50",
        "text-sm",
        "rounded-lg",
        "p-1",
        "focus:ring-blue-500",
        "focus:border-blue-500",
        "block",
        "w-full"
    )
});
pub const NAVBAR_BUTTON_CLASSES: Lazy<Classes> = Lazy::new(|| {
    classes!(
        "appearance-none",
        "bg-teal-500",
        "dark:bg-teal-900",
        "border",
        "border-teal-500",
        "text-teal-50",
        "text-sm",
        "rounded",
        "p-1",
        "focus:ring-blue-500",
        "focus:border-blue-500",
        "block",
    )
});

#[autoprops]
#[function_component(Logo)]
pub fn logo() -> HtmlResult {
    let navigator = use_navigator();
    let logo_html = html! {
        <span class={classes!("font-semibold", "text-xl", "tracking-tight")}>
            { "logo" }
        </span>
    };
    Ok(html! {<GoHome navigator={navigator} html={logo_html}/>})
}

#[autoprops]
#[function_component(Navbar)]
pub fn navbar() -> HtmlResult {
    let placeholder = "https://github.com/hayas1/loc-viewer";

    let navigator = use_navigator();
    let url_input = use_node_ref();
    let submit = {
        let (navigator, url_input) = (navigator.clone(), url_input.clone());
        Callback::from(move |_| {
            let route: Result<_> = (|| {
                let url = url_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let repository = if url.is_empty() {
                    GitHubRepository::from_url(&Url::parse(placeholder).map_err(anyhow::Error::from)?)?
                } else {
                    GitHubRepository::from_url(&Url::parse(&url).map_err(anyhow::Error::from)?)?
                };
                let (host, GitHubRepository { owner, repo }) = (repository.host(), repository);
                Ok(Route::Statistics { host, owner, repo })
            })();
            match (navigator.clone(), route) {
                (None, _) => gloo::console::warn!("Navigator is not available"),
                (Some(navigator), Ok(route)) => navigator.push(&route),
                (_, Err(err)) => gloo::console::error!(err.to_string()), // TODO error handling
            }
        })
    };

    Ok(html! {
        <nav class={classes!("flex", "items-center", "flex-wrap", "text-teal-50", "bg-teal-600", "dark:bg-teal-900", "py-3", "px-6")}>
            <div class={classes!("flex", "justify-between", "items-center", "w-full")}>
                <div class={classes!("inline-block", "text-center")}>
                    <Logo/>
                </div>
                <div class={classes!("inline-block", "shrink", "w-full", "max-w-screen-md")}>
                    <div class={classes!("flex", "justify-center", "text-center")}>
                        <div class={classes!("inline-block", "grow", "px-3")} title={"Repository URL"}>
                            <Icon icon_id={IconId::OcticonsMarkGithub16}
                                class={classes!("absolute", "text-teal-600", "dark:text-teal-50", "m-1")}
                            />
                            <input ref={url_input}
                                class={classes!("ps-8", NAVBAR_INPUT_CLASSES.clone())}
                                onchange={submit}
                                type="text"
                                placeholder={placeholder}
                                aria-label="repository-url"
                            />
                        </div>
                    </div>
                </div>
                <div class={classes!("inline-block", "text-right", "text-teal-200", "text-sm")}>
                    <a href={REPOSITORY}>
                        <Icon icon_id={IconId::HeroiconsOutlineInformationCircle} title={"Information"}/>
                    </a>
                </div>
            </div>
        </nav>
    })
}
