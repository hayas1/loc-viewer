use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};

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

    let input_classes = classes!(
        "appearance-none",
        "bg-teal-50",
        "border",
        "border-teal-700",
        "text-teal-900",
        "text-sm",
        "rounded-lg",
        "p-1",
        "focus:ring-blue-500",
        "focus:border-blue-500",
        "block",
        "w-full"
    );
    Ok(html! {
        // base: https://v1.tailwindcss.com/components/navigation#responsive-header
        <nav class={classes!("relative", "flex", "items-center", "justify-between", "flex-wrap", "text-white", "bg-teal-600", "dark:bg-teal-900", "py-3", "px-6", "h-70px")}>
            <div class={classes!("absolute", "flex", "items-center", "flex-shrink-0", "mr-6")}>
                <div>{"logo"}</div>
            </div>
            <div class={classes!("relative", "flex-grow", "block", "w-full", "md:flex", "md:items-center", "md:w-auto", collapsed.then(|| "hidden"))}>
                <div class={classes!("md:flex-grow")}>
                    <div class={classes!("text-center")}>
                        <div class={classes!("md:inline-block", "md:mt-0")} title={"Host"}>
                            <input
                                class={classes!(input_classes.clone())}
                                type="text"
                                placeholder={"https://github.com"}
                                aria-label="Host"/>
                        </div>
                        <div class={classes!("md:inline-block", "md:mt-0")} title={"Owner"}>
                            <input
                                class={classes!(input_classes.clone())}
                                type="text"
                                placeholder={"owner"}
                                aria-label="Owner"/>
                        </div>
                        <div class={classes!("md:inline-block", "md:mt-0")} title={"Repo"}>
                            <input
                                class={classes!(input_classes.clone())}
                                type="text"
                                placeholder={"repo"}
                                aria-label="Repo"/>
                        </div>
                    </div>
                </div>
            </div>
            <div class={classes!("relative", "flex-grow", "block", "w-full", "text-right", "md:hidden")}>
                <button onclick={toggle}>
                    if *collapsed {
                        <Icon icon_id={IconId::HeroiconsSolidChevronDoubleDown}/>
                    } else {
                        <Icon icon_id={IconId::HeroiconsSolidChevronDoubleUp}/>
                    }
                </button>
            </div>
        </nav>
    })
}
