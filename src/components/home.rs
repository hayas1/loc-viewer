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

pub const REPOSITORY_URL_INPUT_CLASSES: Lazy<Classes> = Lazy::new(|| {
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

pub const REPOSITORY_INFO_INPUT_GROUP_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("pt-4", "h-10", "w-full", "flex", "items-center", "border-b", "border-teal-500"));
pub const REPOSITORY_INFO_INPUT_LABEL_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("w-20", "text-sm", "text-right", "text-teal-500", "dark:text-teal-50")); // TODO width
pub const REPOSITORY_INFO_INPUT_ICON_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("h-5", "m-2", "text-teal-500", "dark:text-teal-50")); // TODO better icon position
pub const REPOSITORY_INFO_INPUT_CLASSES: Lazy<Classes> = Lazy::new(|| {
    classes!(
        "ps-3",
        "appearance-none",
        "border-none",
        "w-full",
        "text-teal-700",
        "dark:text-teal-50",
        "rounded-sm",
        "leading-tight",
        "focus:outline-none"
    )
});

pub const REPOSITORY_INFO_BUTTON_CLASSES: Lazy<Classes> = Lazy::new(|| {
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
                    class={classes!("ps-8", REPOSITORY_URL_INPUT_CLASSES.clone())}
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
    let (sha_input, paths_input, excluded_input) = (use_node_ref(), use_node_ref(), use_node_ref());

    let statistics = {
        let (host_input, owner_input, repo_input) = (host_input.clone(), owner_input.clone(), repo_input.clone());
        let (sha_input, paths_input, excluded_input) = (sha_input.clone(), paths_input.clone(), excluded_input.clone());
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

                // TODO sha and paths and excluded
                let _sha = sha_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let _paths = paths_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let _excluded = excluded_input
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
            <div class={REPOSITORY_INFO_INPUT_GROUP_CLASSES.clone()}>
                <label for="host-input" class={classes!(REPOSITORY_INFO_INPUT_LABEL_CLASSES.clone())}>{"Host"}</label>
                <input ref={host_input}
                    class={classes!(REPOSITORY_INFO_INPUT_CLASSES.clone(), "bg-transparent")}
                    id="host-input"
                    type="text"
                    title={"Repository host"}
                    placeholder={"https://github.com"}
                    aria-label="RepositoryHost"
                />
                <div class={classes!("relative")}>
                    <Icon icon_id={IconId::OcticonsMarkGithub16} class={classes!(REPOSITORY_INFO_INPUT_ICON_CLASSES.clone())}/>
                </div>
            </div>
            <div class={REPOSITORY_INFO_INPUT_GROUP_CLASSES.clone()}>
                <label for="owner-input" class={classes!(REPOSITORY_INFO_INPUT_LABEL_CLASSES.clone())}>{"Owner"}</label>
                <input ref={owner_input}
                    class={classes!(REPOSITORY_INFO_INPUT_CLASSES.clone(), "bg-teal-50", "dark:bg-teal-800")}
                    id="owner-input"
                    type="text"
                    title={"Repository owner"}
                    placeholder={"hayas1"}
                    aria-label="RepositoryOwner"
                />
                <div class={classes!("relative")}>
                    <Icon icon_id={IconId::OcticonsOrganization16} class={classes!(REPOSITORY_INFO_INPUT_ICON_CLASSES.clone())}/>
                </div>
            </div>
            <div class={REPOSITORY_INFO_INPUT_GROUP_CLASSES.clone()}>
                <label for="repo-input" class={classes!(REPOSITORY_INFO_INPUT_LABEL_CLASSES.clone())}>{"Repo"}</label>
                <input ref={repo_input}
                    class={classes!(REPOSITORY_INFO_INPUT_CLASSES.clone(), "bg-teal-50", "dark:bg-teal-800")}
                    id="repo-input"
                    type="text"
                    title={"Repository name"}
                    placeholder={"loc-viewer"}
                    aria-label="RepositoryName"
                />
                <div class={classes!("relative")}>
                    <Icon icon_id={IconId::OcticonsRepo16} class={classes!(REPOSITORY_INFO_INPUT_ICON_CLASSES.clone())}/>
                </div>
            </div>
            <div class={REPOSITORY_INFO_INPUT_GROUP_CLASSES.clone()}>
                <label for="sha-input" class={classes!(REPOSITORY_INFO_INPUT_LABEL_CLASSES.clone())}>{"SHA"}</label>
                <input ref={sha_input}
                    class={classes!(REPOSITORY_INFO_INPUT_CLASSES.clone(), "bg-transparent")}
                    id="sha-input"
                    type="text"
                    title={"SHA of the Repository to get statistics"}
                    placeholder={"main"}
                    aria-label="RepositorySHA"
                />
                <div class={classes!("relative")}>
                    <Icon icon_id={IconId::OcticonsGitCommit16} class={classes!(REPOSITORY_INFO_INPUT_ICON_CLASSES.clone())}/>
                </div>
            </div>
            <div class={REPOSITORY_INFO_INPUT_GROUP_CLASSES.clone()}>
                <label for="paths-input" class={classes!(REPOSITORY_INFO_INPUT_LABEL_CLASSES.clone())}>{"Paths"}</label>
                <input ref={paths_input}
                    class={classes!(REPOSITORY_INFO_INPUT_CLASSES.clone(), "bg-transparent")}
                    id="paths-input"
                    type="text"
                    title={"Paths of the repository to get statistics"}
                    placeholder={"/"}
                    aria-label="RepositoryPaths"
                />
                <div class={classes!("relative")}>
                    <Icon icon_id={IconId::OcticonsFileDirectoryOpenFill16} class={classes!(REPOSITORY_INFO_INPUT_ICON_CLASSES.clone())}/>
                </div>
            </div>
            <div class={REPOSITORY_INFO_INPUT_GROUP_CLASSES.clone()}>
                <label for="excluded-input" class={classes!(REPOSITORY_INFO_INPUT_LABEL_CLASSES.clone())}>{"Excluded"}</label>
                <input ref={excluded_input}
                    class={classes!(REPOSITORY_INFO_INPUT_CLASSES.clone())}
                    id="excluded-input"
                    type="text"
                    title={"Excluded paths of the repository to get statistics"}
                    placeholder={""}
                    aria-label="RepositoryExcluded"
                />
                <div class={classes!("relative")}>
                    <Icon icon_id={IconId::OcticonsSkip16} class={classes!(REPOSITORY_INFO_INPUT_ICON_CLASSES.clone())}/>
                </div>
            </div>
            <div class={classes!("py-2", "text-center", "pt-4", "w-full")}>
                <button
                    class={classes!(REPOSITORY_INFO_BUTTON_CLASSES.clone())}
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
