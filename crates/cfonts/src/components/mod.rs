#[cfg(feature = "dioxus")]
mod dioxus;
#[cfg(feature = "dioxus")]
pub use dioxus::CfontsDioxus;

#[cfg(feature = "leptos")]
mod leptos;
#[cfg(feature = "leptos")]
pub use leptos::CfontsLeptos;

#[cfg(feature = "ratatui")]
mod ratatui;
#[cfg(feature = "ratatui")]
pub use ratatui::CfontsWidget;
