use once_cell::sync::Lazy;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

use super::{home::RepoUrlBar, routes::GoHome, REPOSITORY};

pub const BG_THEME: Lazy<Classes> =
    Lazy::new(|| classes!("text-teal-900", "dark:text-teal-50", "bg-teal-50", "dark:bg-teal-950"));
pub const BG_LINK_THEME: Lazy<Classes> =
    Lazy::new(|| classes!("text-teal-600", "dark:text-teal-400", "hover:text-teal-500"));

#[autoprops]
#[function_component(Logo)]
pub fn logo() -> HtmlResult {
    let navigator = use_navigator();
    Ok(html! {
        <GoHome navigator={navigator}>
            <span class={classes!("font-semibold", "text-xl", "tracking-tight")}>
                { "Toukei" }
            </span>
        </GoHome>
    })
}

#[autoprops]
#[function_component(Navbar)]
pub fn navbar() -> HtmlResult {
    Ok(html! {
        <nav class={classes!("flex", "items-center", "flex-wrap", "text-teal-50", "bg-teal-600", "dark:bg-teal-950", "py-3", "px-6", "sticky", "top-0")}>
            <div class={classes!("flex", "justify-between", "items-center", "w-full")}>
                <div class={classes!("inline-block", "text-center")}>
                    <Logo/>
                </div>
                <div class={classes!("inline-block", "invisible", "md:visible", "px-8", "shrink", "w-full", "max-w-screen-md")}>
                    <RepoUrlBar/>
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

#[autoprops]
#[function_component(Background)]
pub fn background(children: &Children) -> HtmlResult {
    Ok(html! {
        <div class={classes!(BG_THEME.clone())}>
            { children.clone() }
        </div>
    })
}

#[autoprops]
#[function_component(Screen)]
pub fn screen(children: &Children) -> HtmlResult {
    Ok(html! {
        <div class={classes!("flex", "flex-col", "min-h-screen")}>
            <div class={classes!("flex-1", BG_THEME.clone())}>
                { children.clone() }
            </div>
            <Footer/>
        </div>
    })
}

#[autoprops]
#[function_component(Footer)]
pub fn footer() -> HtmlResult {
    Ok(html! {
        <footer class={classes!(BG_THEME.clone(), "text-center", "p-4")}>
            <p>
                { "Powered by " }
                <a href={REPOSITORY}
                    target="_blank" rel="noopener noreferrer"
                    class={classes!(BG_LINK_THEME.clone())}
                >
                    { env!("CARGO_PKG_NAME") }
                </a>
            </p>
        </footer>
    })
}
