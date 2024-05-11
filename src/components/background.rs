use once_cell::sync::Lazy;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::hooks::use_navigator;

use super::{
    darkmode::{NavIconDarkmode, Theme},
    forms::RepoUrlBar,
    routes::GoHome,
    REPOSITORY,
};

pub const BG_THEME: Lazy<Classes> =
    Lazy::new(|| classes!("text-teal-900", "dark:text-teal-50", "bg-teal-50", "dark:bg-teal-950"));
pub const BG_LINK_THEME: Lazy<Classes> =
    Lazy::new(|| classes!("text-teal-600", "dark:text-teal-400", "hover:text-teal-500"));
pub const PANE_THEME: Lazy<Classes> =
    Lazy::new(|| classes!("container", "rounded-xl", "bg-white", "dark:text-teal-50", "dark:bg-teal-900"));

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
        // TODO if we remove z-[100], navbar will be overlaid by another "absolute" element
        <nav class={classes!("absolute", "z-[100]", "flex", "items-center", "flex-wrap", "text-teal-50", "bg-teal-600", "dark:bg-teal-950", "py-3", "px-6", "sticky", "top-0")}>
            <div class={classes!("flex", "justify-between", "items-center", "w-full")}>
                <div class={classes!("inline-block", "text-center")}>
                    <Logo/>
                </div>
                <div class={classes!("inline-block", "invisible", "md:visible", "px-8", "shrink", "w-full", "max-w-screen-md")}>
                    <RepoUrlBar/>
                </div>
                <div class={classes!("inline-block", "text-center", "flex", "items-end")}>
                    <div class={classes!("text-teal-200", "text-sm")}>
                        <NavIconDarkmode/>
                    </div>
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
#[function_component(ResponsivePanesFrame)]
pub fn responsive_panes_frame(children: &Children) -> HtmlResult {
    Ok(html! {
        <div class={classes!("p-2", "flex", "justify-center")}>
            <div class={classes!("pt-4", "flex", "flex-col", "justify-center", "items-center",
                "md:flex-row", "md:justify-center", "md:items-start", "gap-4", "w-full", "max-w-screen-xl"
            )}>
                { children.clone() }
            </div>
        </div>
    })
}

#[autoprops]
#[function_component(Pane)]
pub fn pane(children: &Children, #[prop_or_default] class: Classes) -> HtmlResult {
    Ok(html! {
        <div class={classes!(PANE_THEME.clone(), class.clone())}>
            { children.clone() }
        </div>
    })
}

#[autoprops]
#[function_component(Screen)]
pub fn screen(children: &Children) -> HtmlResult {
    let class = use_context::<UseReducerHandle<Theme>>().map(|t| t.class()).unwrap_or_else(|| Theme::default().class());
    Ok(html! {
        <div class={classes!("flex", "flex-col", "min-h-screen", class)}>
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
        <footer class={classes!(BG_THEME.clone(), "text-xs", "text-center", "p-4")}>
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
