use leptos::*;

use crate::repositories::color_repository::{all_colors, AppendColor, RemoveColor};

#[component]
pub fn ColoHome() -> impl IntoView {
    view! {
        <div>
            <header>
                <h2>Color Tool</h2>
            </header>
        </div>
    }
}