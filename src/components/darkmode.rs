use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_icons::{Icon, IconId};

use crate::error::Result;

use super::STORAGE_KEY_DARKMODE;

#[derive(Debug, Clone, Eq, PartialEq)]
/// TODO use_darkmode hook
pub enum DarkmodeConfig {
    Light,
    Dark,
    System,
}

impl DarkmodeConfig {
    fn read_local_storage() -> Option<bool> {
        LocalStorage::get(STORAGE_KEY_DARKMODE).ok()
    }

    fn write_local_storage(&self) -> Result<()> {
        Ok(match self {
            Self::Light => LocalStorage::set(STORAGE_KEY_DARKMODE, false).map_err(anyhow::Error::from)?,
            Self::Dark => LocalStorage::set(STORAGE_KEY_DARKMODE, true).map_err(anyhow::Error::from)?,
            Self::System => LocalStorage::delete(STORAGE_KEY_DARKMODE),
        })
    }

    pub fn get() -> Self {
        match Self::read_local_storage() {
            Some(true) => Self::Dark,
            Some(false) => Self::Light,
            None => Self::System,
        }
    }

    pub fn save(&self) -> Result<&Self> {
        Self::write_local_storage(self)?;
        Ok(self)
    }

    pub fn theme(&self) -> Theme {
        Theme::from_config(self)
    }

    pub fn icon_id(&self) -> IconId {
        match self {
            Self::Light => IconId::HeroiconsOutlineSun,
            Self::Dark => IconId::HeroiconsOutlineMoon,
            Self::System => IconId::HeroiconsOutlineComputerDesktop,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::System => "System",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}
impl Theme {
    fn read_system() -> Result<bool> {
        let query = "(prefers-color-scheme: dark)";
        let system = gloo::utils::window()
            .match_media(query)
            .map_err(|_| anyhow::anyhow!("Failed match media query"))? // TODO error handling
            .ok_or_else(|| anyhow::anyhow!("Do not get MediaQueryList"))? // TODO error handling
            .matches();
        Ok(system)
    }

    fn from_config(config: &DarkmodeConfig) -> Self {
        match config {
            DarkmodeConfig::Light => Self::Light,
            DarkmodeConfig::Dark => Self::Dark,
            DarkmodeConfig::System => match Self::read_system() {
                Ok(true) => Self::Dark,
                _ => Self::Light,
            },
        }
    }

    pub fn get() -> Self {
        Self::from_config(&DarkmodeConfig::get())
    }

    pub fn class(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn icon_id(&self) -> IconId {
        match self {
            Self::Light => DarkmodeConfig::Light.icon_id(),
            Self::Dark => DarkmodeConfig::Dark.icon_id(),
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Light => "Light mode",
            Self::Dark => "Dark mode",
        }
    }
}

impl Reducible for Theme {
    type Action = DarkmodeConfig;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        action.theme().into()
    }
}

#[autoprops]
#[function_component(NavIconDarkmode)]
pub fn nav_icon_darkmode() -> HtmlResult {
    let theme = use_context::<UseReducerHandle<Theme>>().unwrap(); // TODO unreachable
    let current = DarkmodeConfig::get();

    Ok(html! {
        <div class={classes!("group")}>
            <button class={classes!()}>
                <Icon icon_id={theme.icon_id()} title={theme.title()}/>
            </button>
            <div class={classes!("hidden", "group-focus-within:block")}>
                <div class={classes!("flex", "justify-end", "absolute", "top-0", "left-0", "w-full", "h-full")}>
                    <div class={classes!("block", "mt-14", "mx-4")}>
                        <ul class={classes!("container", "rounded-lg", "text-base", "border-2",
                            "text-teal-700", "bg-teal-50", "border-teal-100",
                            "dark:text-teal-50", "dark:bg-teal-900", "dark:border-teal-800",
                        )}>
                            {for [DarkmodeConfig::Light, DarkmodeConfig::Dark, DarkmodeConfig::System].into_iter().map(|c| {
                                html! {
                                    <NavIconDarkmodeSelect config={c} current={current.clone()}/>
                                }
                            })}
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    })
}

#[autoprops]
#[function_component(NavIconDarkmodeSelect)]
pub fn nav_icon_darkmode_select(config: &DarkmodeConfig, current: &DarkmodeConfig) -> HtmlResult {
    let theme = use_context::<UseReducerHandle<Theme>>().unwrap(); // TODO unreachable

    let save = {
        let (theme, config) = (theme.clone(), config.clone());
        move |_| theme.dispatch(config.save().unwrap().clone())
    };

    Ok(html! {
        <li>
            <button
                onclick={save}
                class={classes!("flex", "justify-start", "py-1", "px-2", "w-full",
                    "hover:bg-teal-100", "dark:hover:bg-teal-800",
                    (current == config).then(|| classes!("font-bold", "text-teal-300")),
                )}
                title={config.title()}
            >
                <Icon icon_id={config.icon_id()}/>
                <span class={classes!("ml-2")}>{config.title()}</span>
            </button>
        </li>
    })
}
