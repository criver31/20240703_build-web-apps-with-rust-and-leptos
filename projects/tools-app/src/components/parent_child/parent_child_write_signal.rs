use leptos::*;

#[component]
pub fn Parent() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);

    let increment = |_| set_counter.update(|c| *c += 1);
    let decrement = |_| set_counter.update(|c| *c -= 1);

    view! { <p>"Parent Child Write Signal."</p> }
}
