use crate::components::ui::theme_toggle::ThemeToggle;
use crate::utils::theme::Theme;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub current_theme: Theme,
    pub on_theme_change: Callback<Theme>,
}

#[styled_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header class="bg-white dark:bg-gray-900 shadow-lg border-b-2 border-orange-500">
            <div class="container mx-auto px-4 py-6">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4">
                        <div class="w-12 h-12 bg-orange-500 rounded-full flex items-center justify-center">
                            <span class="text-white font-bold text-xl">{"₿"}</span>
                        </div>
                        <div>
                            <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
                                {"Bitcoin Wealth Comparison"}
                            </h1>
                            <p class="text-sm text-gray-600 dark:text-gray-300">
                                {"Compare your Bitcoin wealth against global distribution"}
                            </p>
                        </div>
                    </div>

                    <div class="hidden md:flex items-center space-x-6">
                        <nav class="flex space-x-6">
                            <a href="#dashboard" class="text-gray-600 dark:text-gray-300 hover:text-orange-500 transition-colors">
                                {"Dashboard"}
                            </a>
                            <a href="#about" class="text-gray-600 dark:text-gray-300 hover:text-orange-500 transition-colors">
                                {"About"}
                            </a>
                            <a href="#privacy" class="text-gray-600 dark:text-gray-300 hover:text-orange-500 transition-colors">
                                {"Privacy"}
                            </a>
                        </nav>

                        <div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
                            <span class="w-2 h-2 bg-green-500 rounded-full"></span>
                            <span>{"All calculations client-side"}</span>
                        </div>

                        // Theme toggle
                        <ThemeToggle
                            current_theme={props.current_theme}
                            on_theme_change={props.on_theme_change.clone()}
                        />
                    </div>

                    // Mobile menu button and theme toggle for mobile
                    <div class="md:hidden flex items-center space-x-2">
                        <ThemeToggle
                            current_theme={props.current_theme}
                            on_theme_change={props.on_theme_change.clone()}
                        />
                        <button class="p-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>
                    </div>

                // Mobile menu (hidden by default)
                <div class="md:hidden mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                    <nav class="flex flex-col space-y-2">
                        <a href="#dashboard" class="py-2 px-3 text-gray-600 dark:text-gray-300 hover:text-orange-500 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-lg transition-colors">
                            {"Dashboard"}
                        </a>
                        <a href="#about" class="py-2 px-3 text-gray-600 dark:text-gray-300 hover:text-orange-500 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-lg transition-colors">
                            {"About"}
                        </a>
                        <a href="#privacy" class="py-2 px-3 text-gray-600 dark:text-gray-300 hover:text-orange-500 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-lg transition-colors">
                            {"Privacy"}
                        </a>
                    </nav>

                    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                        <div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
                            <span class="w-2 h-2 bg-green-500 rounded-full"></span>
                            <span>{"All calculations performed locally"}</span>
                        </div>
                    </div>
                </div>
                </div>
            </div>
        </header>
    }
}
