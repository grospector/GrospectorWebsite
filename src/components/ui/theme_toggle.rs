use crate::utils::theme::Theme;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    pub current_theme: Theme,
    pub on_theme_change: Callback<Theme>,
}

#[styled_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
    let onclick = {
        let current_theme = props.current_theme;
        let on_theme_change = props.on_theme_change.clone();

        Callback::from(move |_: MouseEvent| {
            let new_theme = current_theme.toggle();
            on_theme_change.emit(new_theme);
        })
    };

    let (icon, tooltip) = match props.current_theme {
        Theme::Light => ("🌙", "Switch to dark mode"),
        Theme::Dark => ("☀️", "Switch to light mode"),
    };

    html! {
        <button
            class="relative inline-flex items-center justify-center w-10 h-10 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 transition-all duration-200 group"
            onclick={onclick}
            title={tooltip}
            aria-label={tooltip}
        >
            <span class="text-lg transition-transform duration-200 group-hover:scale-110">
                {icon}
            </span>

            // Tooltip
            <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 text-xs text-white bg-gray-900 dark:bg-gray-700 rounded opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none whitespace-nowrap">
                {tooltip}
                <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900 dark:border-t-gray-700"></div>
            </div>
        </button>
    }
}
