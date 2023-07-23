use crate::model::conversation::Conversation;
use leptos::{html::Div, *};

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {cx,
      <div class="chat-container" node_ref=chat_div_ref>
      {move || conversation.get().messages.iter().map( move |message| {

        let class_str = if message.user {
          "msg-user"
        } else {
          "msg-model"
        };

        let container_class = if message.user {
          "container-user"
        } else {
          "container-model"
        };

        let img_class = if !message.user {
          "chad-msg-icon"
        } else {
          "hidden"
        };

        view! {cx,
          <div class={format!("msg-container {}", container_class)}>
            <img class={format!("chad-img {}", img_class)} src="https://i.imgur.com/3gITj2O.png" />
            <div class={class_str}>
              {message.text.clone()}
            </div>
          </div>
        }
      }).collect::<Vec<_>>()
      }
      </div>
    }
}
