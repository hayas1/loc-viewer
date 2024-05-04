use once_cell::sync::Lazy;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};

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

#[autoprops]
#[function_component(Navbar)]
pub fn navbar() -> HtmlResult {
    let collapsed = use_state(|| true);
    let toggle = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| {
            collapsed.set(!*collapsed);
        })
    };

    Ok(html! {
        // base: https://v1.tailwindcss.com/components/navigation#responsive-header
        <nav class={classes!("flex", "items-center", "flex-wrap", "text-white", "bg-teal-600", "dark:bg-teal-900", "py-3", "px-6")}>
            <div class={classes!("flex", "justify-between", "items-center", "w-full")}>
                <div class={classes!("inline-block", "text-center")}>
                    {"logo"}
                </div>
                <div class={classes!("inline-block", "justify-center", "w-full")}>
                    <div class={classes!("flex", "justify-center", "text-center")}>
                        <div class={classes!("inline-block", "mt-0")} title={"Host"}>
                            <NavbarHostInput/>
                        </div>
                        <div class={classes!("inline-block", "mt-0")} title={"Owner"}>
                            <input
                                class={classes!(NAVBAR_INPUT_CLASSES.clone())}
                                type="text"
                                placeholder={"owner"}
                                aria-label="Owner"/>
                        </div>
                        <div class={classes!("inline-block", "mt-0")} title={"Repo"}>
                            <input
                                class={classes!(NAVBAR_INPUT_CLASSES.clone())}
                                type="text"
                                placeholder={"repo"}
                                aria-label="Repo"/>
                        </div>
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
pub fn navbar_host_input() -> HtmlResult {
    Ok(html! {
       <div>
           <Icon icon_id={IconId::OcticonsMarkGithub16} class={classes!("absolute", "text-teal-500", "m-1")}/>
           <input
                class={classes!(NAVBAR_INPUT_CLASSES.clone(), "ps-7")}
                type="text"
                placeholder={"https://github.com"}
                aria-label="Host"/>
       </div>
    })
}
