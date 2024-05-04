use once_cell::sync::Lazy;
use url::Url;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

use super::routes::Route;
use crate::{
    error::{render::Unreachable, Result},
    github::repository::GitHubRepository,
};

pub const NAVBAR_INPUT_CLASSES: Lazy<Classes> = Lazy::new(|| {
    classes!(
        "appearance-none",
        "bg-teal-50",
        "dark:bg-teal-900",
        "border",
        "border-teal-700",
        "text-teal-900",
        "dark:text-teal-50",
        "text-sm",
        "rounded-lg",
        "p-1",
        "focus:outline-none",
        "block",
        "w-full"
    )
});

pub const HOME_INPUT_GROUP_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("pt-4", "h-10", "w-full", "flex", "items-center", "border-b", "border-teal-500"));
pub const HOME_INPUT_LABEL_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("w-16", "text-right", "text-teal-500", "dark:text-teal-50")); // TODO width
pub const HOME_INPUT_CLASSES: Lazy<Classes> = Lazy::new(|| {
    classes!(
        "ps-3",
        "appearance-none",
        "bg-transparent",
        "border-none",
        "w-full",
        "text-teal-700",
        "dark:text-teal-50",
        "leading-tight",
        "focus:outline-none"
    )
});

pub const HOME_BUTTON_CLASSES: Lazy<Classes> = Lazy::new(|| {
    classes!(
        "p-2",
        "bg-gradient-to-r",
        "from-teal-500",
        "via-teal-600",
        "to-teal-700",
        "hover:bg-gradient-to-br",
        "border-teal-600",
        "hover:border-teal-200",
        "focus:outline-none",
        "focus:border-teal-200",
        "text-sm",
        "border-2",
        "text-white",
        "rounded-full"
    )
});

#[function_component(Home)]
pub fn home() -> HtmlResult {
    let h1 = "Statistics about code";
    let description =
        "Input repository URL or information, to display statistics about code of the remote repositories.";
    Ok(html! {
        <div class={classes!("p-2", "flex", "justify-center", "min-h-screen", "bg-teal-0", "bg-teal-50", "dark:bg-teal-950")}>
            <div class={classes!("p-6", "container", "rounded-xl", "max-w-screen-lg", "bg-white", "dark:text-teal-50", "dark:bg-teal-900")}>
                <h1 class={classes!("p-2", "text-teal-900", "dark:text-teal-50", "text-xl")}>
                    { h1 }
                </h1>
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

#[autoprops]
#[function_component(RepoUrlBar)]
pub fn repo_url_bar() -> HtmlResult {
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
                (None, _) => gloo::console::error!("Navigator is not available"),
                (Some(navigator), Ok(route)) => navigator.push(&route),
                (_, Err(err)) => gloo::console::error!(err.to_string()), // TODO error handling
            }
        })
    };

    Ok(html! {
        <div class={classes!("flex", "justify-center", "text-center", "w-full")}>
            <div class={classes!("inline-block", "grow")} title={"Repository URL"}>
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
    })
}

#[autoprops]
#[function_component(RepoInfoForms)]
pub fn repo_info_forms() -> HtmlResult {
    let navigator = use_navigator();

    let (host_input, owner_input, repo_input) = (use_node_ref(), use_node_ref(), use_node_ref());
    let (sha_input, path_input) = (use_node_ref(), use_node_ref());

    let statistics = {
        let (host_input, owner_input, repo_input) = (host_input.clone(), owner_input.clone(), repo_input.clone());
        let (sha_input, path_input) = (sha_input.clone(), path_input.clone());
        Callback::from(move |_| {
            let route: Result<Route> = (|| {
                let host = host_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let host = if host.is_empty() { "github".to_string() } else { host };

                let owner = owner_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let owner = if owner.is_empty() { "hayas1".to_string() } else { owner };

                let repo = repo_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let repo = if repo.is_empty() { "loc-viewer".to_string() } else { repo };

                // TODO sha and path
                let _sha = sha_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let _path = path_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();

                Ok(Route::Statistics { host, owner, repo })
            })();
            match (navigator.clone(), route) {
                (None, _) => gloo::console::error!("Navigator is not available"),
                (Some(navigator), Ok(route)) => navigator.push(&route),
                (_, Err(err)) => gloo::console::error!(err.to_string()), // TODO error handling
            }
        })
    };

    Ok(html! {
        <div class={classes!("flex", "flex-wrap", "w-full")}>
            <div class={HOME_INPUT_GROUP_CLASSES.clone()}>
                <label for="host-input" class={classes!(HOME_INPUT_LABEL_CLASSES.clone())}>{"Host"}</label>
                <input ref={host_input}
                    class={classes!(HOME_INPUT_CLASSES.clone())}
                    id="host-input"
                    type="text"
                    title={"Repository host"}
                    placeholder={"https://github.com"}
                    aria-label="RepositoryHost"
                />
            </div>
            <div class={HOME_INPUT_GROUP_CLASSES.clone()}>
                <label for="owner-input" class={classes!(HOME_INPUT_LABEL_CLASSES.clone())}>{"Owner"}</label>
                <input ref={owner_input}
                    class={classes!(HOME_INPUT_CLASSES.clone())}
                    id="owner-input"
                    type="text"
                    title={"Repository owner"}
                    placeholder={"hayas1"}
                    aria-label="RepositoryOwner"
                />
            </div>
            <div class={HOME_INPUT_GROUP_CLASSES.clone()}>
                <label for="repo-input" class={classes!(HOME_INPUT_LABEL_CLASSES.clone())}>{"Repo"}</label>
                <input ref={repo_input}
                    class={classes!(HOME_INPUT_CLASSES.clone())}
                    id="repo-input"
                    type="text"
                    title={"Repository name"}
                    placeholder={"loc-viewer"}
                    aria-label="RepositoryName"
                />
            </div>
            <div class={HOME_INPUT_GROUP_CLASSES.clone()}>
                <label for="sha-input" class={classes!(HOME_INPUT_LABEL_CLASSES.clone())}>{"SHA"}</label>
                <input ref={sha_input}
                    class={classes!(HOME_INPUT_CLASSES.clone())}
                    id="sha-input"
                    type="text"
                    title={"SHA of the Repository to get statistics"}
                    placeholder={"main"}
                    aria-label="RepositorySHA"
                />
            </div>
            <div class={HOME_INPUT_GROUP_CLASSES.clone()}>
                <label for="path-input" class={classes!(HOME_INPUT_LABEL_CLASSES.clone())}>{"Path"}</label>
                <input ref={path_input}
                    class={classes!(HOME_INPUT_CLASSES.clone())}
                    id="path-input"
                    type="text"
                    title={"Path of the repository to get statistics"}
                    placeholder={"/"}
                    aria-label="RepositoryPath"
                />
            </div>
            <div class={classes!("py-2", "text-center", "pt-4", "w-full")}>
                <button
                    class={classes!(HOME_BUTTON_CLASSES.clone())}
                    onclick={statistics}
                    type="button"
                    title={"Get statistics!"}
                >
                    { "Toukei" }
                </button>
            </div>
        </div>
    })
}
