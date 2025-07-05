use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-gray-800 text-white py-12">
            <div class="container mx-auto px-4">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
                    // About section
                    <div class="col-span-1 md:col-span-2">
                        <div class="flex items-center space-x-3 mb-4">
                            <div class="w-8 h-8 bg-bitcoin-orange rounded-full flex items-center justify-center">
                                <span class="text-white font-bold text-lg">{"₿"}</span>
                            </div>
                            <h3 class="text-xl font-bold">{"Bitcoin Wealth Comparison"}</h3>
                        </div>
                        <p class="text-gray-300 mb-4 max-w-md">
                            {"A privacy-first tool to compare your Bitcoin wealth against global distribution. 
                            All calculations are performed locally in your browser."}
                        </p>
                        <div class="flex items-center space-x-4 text-sm">
                            <div class="flex items-center space-x-2">
                                <span class="w-2 h-2 bg-green-500 rounded-full"></span>
                                <span class="text-gray-400">{"Privacy Protected"}</span>
                            </div>
                            <div class="flex items-center space-x-2">
                                <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
                                <span class="text-gray-400">{"Open Source"}</span>
                            </div>
                        </div>
                    </div>
                    
                    // Quick Links
                    <div>
                        <h4 class="text-lg font-semibold mb-4">{"Quick Links"}</h4>
                        <ul class="space-y-2">
                            <li><a href="#dashboard" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"Dashboard"}</a></li>
                            <li><a href="#about" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"About"}</a></li>
                            <li><a href="#methodology" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"Methodology"}</a></li>
                            <li><a href="#faq" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"FAQ"}</a></li>
                        </ul>
                    </div>
                    
                    // Data & Privacy
                    <div>
                        <h4 class="text-lg font-semibold mb-4">{"Data & Privacy"}</h4>
                        <ul class="space-y-2">
                            <li><a href="#privacy" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"Privacy Policy"}</a></li>
                            <li><a href="#data-sources" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"Data Sources"}</a></li>
                            <li><a href="#security" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"Security"}</a></li>
                            <li><a href="#disclaimer" class="text-gray-300 hover:text-bitcoin-orange transition-colors">{"Disclaimer"}</a></li>
                        </ul>
                    </div>
                </div>
                
                <div class="border-t border-gray-700 mt-8 pt-8">
                    <div class="flex flex-col md:flex-row items-center justify-between">
                        <div class="text-gray-400 text-sm mb-4 md:mb-0">
                            {"© 2024 Bitcoin Wealth Comparison. Built with Rust 🦀 and WebAssembly."}
                        </div>
                        
                        <div class="flex items-center space-x-6">
                            <div class="text-sm text-gray-400">
                                {"Data provided by BitInfoCharts"}
                            </div>
                            <div class="flex items-center space-x-4">
                                <a href="https://github.com/your-repo" class="text-gray-400 hover:text-bitcoin-orange transition-colors">
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
                                    </svg>
                                </a>
                                <a href="https://twitter.com/your-handle" class="text-gray-400 hover:text-bitcoin-orange transition-colors">
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
                                    </svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}