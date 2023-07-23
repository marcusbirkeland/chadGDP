use leptos::{*};

#[component]
pub fn Header(cx:Scope) -> impl IntoView {
  view! {cx,
    <div class="header">
      <div class="header-title">
        <img class="chad-img" src="https://i.imgur.com/3gITj2O.png" />
        <h1>"Chad" <span class="GDP">"GDP " </span> "💸"</h1>
      </div>
      <div class="header-details">
        <h2>"A "
        <span class="brown">
          "SHIT"
        </span>
        " language model"</h2>
        <h3 class="shit">
          <span class="S">"Self"</span>
          <span class="H">"Hosted"</span>
          <span class="I">"Imbecile"</span>
          <span class="T">"Transformer"</span>
        </h3>
      </div>
    </div>
  }
}