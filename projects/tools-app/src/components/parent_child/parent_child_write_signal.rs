use leptos::*;

#[component]
pub fn Parent() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px"></div>
            <h3>"Parent Child Write Signal."</h3>
            <p>"Counter: " {counter}</p>
            <div>
                <button type="button" on:click=increment_counter>
                    "Parent + 1"
                </button>
                <button type="button" on:click=decrement_counter>
                    "Parent - 1"
                </button>
            </div>
        </div>
    }
}
