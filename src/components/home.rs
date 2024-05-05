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
                    class={classes!(
                        "ps-8", "appearance-none", "bg-teal-50", "dark:bg-teal-800",
                        "border", "border-teal-700", "text-teal-900", "dark:text-teal-50", "text-sm",
                        "rounded-lg", "p-1", "focus:outline-none", "block", "w-full"
                    )}
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

    let inputs = vec![
        (
            host_input,
            "host-input",
            "Host",
            "Repository host",
            "https://github.com",
            "RepositoryHost",
            false,
            IconId::OcticonsMarkGithub16,
        ),
        (
            owner_input,
            "owner-input",
            "Owner",
            "Repository owner",
            "hayas1",
            "RepositoryOwner",
            true,
            IconId::OcticonsOrganization16,
        ),
        (
            repo_input,
            "repo-input",
            "Repo",
            "Repository name",
            "loc-viewer",
            "RepositoryName",
            true,
            IconId::OcticonsRepo16,
        ),
        (
            sha_input,
            "sha-input",
            "SHA",
            "SHA of the Repository to get statistics",
            "main",
            "RepositorySHA",
            false,
            IconId::OcticonsGitCommit16,
        ),
        (
            paths_input,
            "paths-input",
            "Paths",
            "Paths of the repository to get statistics",
            "/",
            "RepositoryPaths",
            false,
            IconId::OcticonsFileDirectoryOpenFill16,
        ),
        (
            excluded_input,
            "excluded-input",
            "Excluded",
            "Excluded paths of the repository to get statistics",
            "",
            "RepositoryExcluded",
            false,
            IconId::OcticonsSkip16,
        ),
    ];

    Ok(html! {
        <div class={classes!("flex", "flex-wrap", "w-full")}>
            {for inputs.into_iter().map(|(input, id, label, title, placeholder, aria_label, required, icon)| {
                let bg = if required { classes!("bg-teal-50", "dark:bg-teal-800") } else { classes!("bg-transparent") };
                html! {
                    <div class={classes!("pt-4", "h-10", "w-full", "flex", "items-center", "border-b", "border-teal-500")}>
                        <label for={id} class={classes!("w-20", "text-sm", "text-right", "text-teal-500", "dark:text-teal-50")}>
                            {label}
                        </label>
                        <input ref={input}
                            class={classes!(
                                "ps-3", "appearance-none", "border-none", "w-full", "text-teal-700", "dark:text-teal-50",
                                "rounded-sm", "leading-tight", "focus:outline-none", bg
                            )}
                            id={id}
                            type="text"
                            title={title}
                            placeholder={placeholder}
                            aria-label={aria_label}
                        />
                        <Icon icon_id={icon} class={classes!("h-5", "m-2", "text-teal-500", "dark:text-teal-50")}/>
                    </div>
                }
            })}
            <div class={classes!("py-2", "text-center", "pt-4", "w-full")}>
                <button
                    class={classes!(
                        "p-2", "bg-gradient-to-r", "from-teal-500", "via-teal-600", "to-teal-700", "hover:bg-gradient-to-br",
                        "border-teal-600", "hover:border-teal-200", "focus:outline-none", "focus:border-teal-200", "text-sm",
                        "border-2", "text-white", "rounded-full"
                    )}
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
