use once_cell::sync::Lazy;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

use crate::error::{render::Unreachable, Result};

use super::routes::{Route, RouterUnavailable};

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
    Ok(html! {
        <span class={classes!("font-semibold", "text-xl", "tracking-tight")}>
            { "logo" }
        </span>
    })
}

#[autoprops]
#[function_component(Navbar)]
pub fn navbar() -> HtmlResult {
    let Some(navigator) = use_navigator() else {
        return Ok(html! { <RouterUnavailable/> });
    };

    let collapsed = use_state(|| true);
    let toggle = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| {
            collapsed.set(!*collapsed);
        })
    };

    let (host_input, owner_input, repo_input) = (use_node_ref(), use_node_ref(), use_node_ref());
    let on_click = {
        let (host_input, owner_input, repo_input) = (host_input.clone(), owner_input.clone(), repo_input.clone());
        Callback::from(move |_| {
            let route: Result<_> = (|| {
                let host = host_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let owner = owner_input
                    .cast::<HtmlInputElement>()
                    .ok_or_else(|| anyhow::anyhow!(Unreachable::DomMaybeChanged))?
                    .value();
                let repo = repo_input
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
        <nav class={classes!("flex", "items-center", "flex-wrap", "text-white", "bg-teal-600", "dark:bg-teal-900", "py-3", "px-6")}>
            <div class={classes!("flex", "justify-between", "items-center", "w-full")}>
                <div class={classes!("inline-block", "text-center")}>
                    <Logo/>
                </div>
                <div class={classes!("inline-block", "justify-center", "w-full")}>
                    <div class={classes!("flex", "justify-center", "text-center")}>
                        <div class={classes!("inline-block", "mt-0")} title={"Host"}>
                            <NavbarHostInput input_ref={host_input}/>
                        </div>
                        <div class={classes!("inline-block", "mt-0")} title={"Owner"}>
                            <input ref={owner_input}
                                class={classes!(NAVBAR_INPUT_CLASSES.clone())}
                                type="text"
                                placeholder={"owner"}
                                aria-label="Owner"/>
                        </div>
                        <div class={classes!("inline-block", "mt-0")} title={"Repo"}>
                            <input ref={repo_input}
                                class={classes!(NAVBAR_INPUT_CLASSES.clone())}
                                type="text"
                                placeholder={"repo"}
                                aria-label="Repo"/>
                        </div>
                        <button
                            class={classes!("inline-block", "mt-0", NAVBAR_BUTTON_CLASSES.clone())}
                            onclick={on_click}
                            type="button"
                        >
                            { "Toukei" }
                        </button>
                    </div>
                </div>
                <div class={classes!("inline-block", "text-right")}>
                    <button onclick={toggle}>
                        if *collapsed {
                            <Icon icon_id={IconId::HeroiconsSolidChevronDoubleDown}/>
                        } else {
                            <Icon icon_id={IconId::HeroiconsSolidChevronDoubleUp}/>
                        }
                    </button>
                </div>
            </div>
        </nav>
    })
}

#[autoprops]
#[function_component(NavbarHostInput)]
pub fn navbar_host_input(input_ref: &NodeRef) -> HtmlResult {
    Ok(html! {
       <div>
           <Icon icon_id={IconId::OcticonsMarkGithub16} class={classes!("absolute", "text-teal-500", "m-1")}/>
           <input ref={input_ref}
                class={classes!(NAVBAR_INPUT_CLASSES.clone(), "ps-7")}
                type="text"
                placeholder={"https://github.com"}
                aria-label="Host"/>
       </div>
    })
}
