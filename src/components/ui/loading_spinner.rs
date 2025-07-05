use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, PartialEq)]
pub struct LoadingSpinnerProps {
    #[prop_or_default]
    pub size: SpinnerSize,
    #[prop_or_default]
    pub message: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum SpinnerSize {
    #[allow(dead_code)]
    Small,
    Medium,
    Large,
}

impl Default for SpinnerSize {
    fn default() -> Self {
        SpinnerSize::Medium
    }
}

#[styled_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingSpinnerProps) -> Html {
    let (size_class, container_class) = match props.size {
        SpinnerSize::Small => ("w-4 h-4", ""),
        SpinnerSize::Medium => ("w-8 h-8", ""),
        SpinnerSize::Large => ("w-12 h-12", "flex flex-col items-center justify-center min-h-32"),
    };

    html! {
        <div class={format!("flex items-center justify-center {}", container_class)}>
            <div class="flex flex-col items-center space-y-4">
                <div class={format!("loading-spinner {}", size_class)}></div>
                if let Some(message) = &props.message {
                    <p class="text-gray-600 text-sm font-medium">{message}</p>
                }
            </div>
        </div>
    }
}

#[styled_component(LoadingOverlay)]
pub fn loading_overlay() -> Html {
    html! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div class="bg-white rounded-lg p-8 max-w-sm w-full mx-4">
                <LoadingSpinner size={SpinnerSize::Large} message={"Loading Bitcoin data..."} />
            </div>
        </div>
    }
}

#[styled_component(LoadingCard)]
pub fn loading_card() -> Html {
    html! {
        <div class="chart-container p-6">
            <div class="animate-pulse">
                <div class="h-4 bg-gray-200 rounded w-3/4 mb-4"></div>
                <div class="h-64 bg-gray-200 rounded mb-4"></div>
                <div class="h-4 bg-gray-200 rounded w-1/2"></div>
            </div>
        </div>
    }
}
