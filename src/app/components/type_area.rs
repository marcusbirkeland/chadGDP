use leptos::{*, html::Input};

#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
  let input_ref = create_node_ref::<Input>(cx);
  view! {cx,
    <div>
      <form
        class="text-form"
        on:submit=move |ev| {
          ev.prevent_default();
          let input = input_ref.get().expect("input to exist");
          send.dispatch(input.value());
          input.set_value("");
        }
      >
        <input placeholder="Do anything now!" class="chat-input" type="text" node_ref=input_ref/>
        <button class="submit-btn" type="submit">"Send"</button>
      </form>
    </div>
  }
}