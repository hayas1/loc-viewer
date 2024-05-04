use once_cell::sync::Lazy;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use super::routes::{Route, RouterUnavailable};
use crate::error::{render::Unreachable, Result};

pub const HOME_INPUT_GROUP_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("pt-4", "h-10", "w-full", "flex", "items-center", "border-b", "border-teal-500"));
pub const HOME_INPUT_LABEL_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("w-12", "text-teal-500", "dark:text-teal-50"));
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
    let Some(navigator) = use_navigator() else {
        return Ok(html! { <RouterUnavailable/> });
    };

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
            match route {
                Ok(route) => navigator.push(&route),
                Err(err) => gloo::console::error!(err.to_string()), // TODO error handling
            }
        })
    };

    Ok(html! {
        <div class={classes!("p-2", "flex", "justify-center", "min-h-screen", "bg-teal-0", "bg-teal-50", "dark:bg-teal-950")}>
            <div class={classes!("p-6", "md:p-12", "container", "rounded-xl", "max-w-screen-lg", "bg-white", "dark:text-teal-50", "dark:bg-teal-900")}>
                <div class={classes!("md:flex", "gap-4")}>
                    <div class={classes!("md:flex-initial", "md:w-4/12")}>
                        <div class={classes!("text-slate-500", "text-sm")}>
                            {"Display statistics about code of remote the repositories. Let's try Toukei."}
                        </div>
                        <div class={classes!("py-2","text-center")}>
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
                    <div class={classes!("flex", "flex-wrap", "w-full")}>
                        <div class={HOME_INPUT_GROUP_CLASSES.clone()}>
                            <label for="host-input" class={classes!(HOME_INPUT_LABEL_CLASSES.clone())}>{"Host"}</label>
                            <input ref={host_input}
                                class={classes!(HOME_INPUT_CLASSES.clone())}
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
                                type="text"
                                title={"Path of the repository to get statistics"}
                                placeholder={"/"}
                                aria-label="RepositoryPath"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    })
}
