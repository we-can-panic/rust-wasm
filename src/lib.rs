mod model;

use yew::prelude::*;
use yew::events::KeyboardEvent;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use model::TypingGame;

#[function_component(App)]
fn app() -> Html {
    let game = use_state(TypingGame::new);
    let input_ref = use_node_ref();

    let on_keydown = {
        let game = game.clone();
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key().chars().nth(0);
            if let Some(c) = key {
                game.set(game.on_key_input(c));
            }
        })
    };

    {
        let input_ref = input_ref.clone();
        use_effect(move || {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().ok();
            }
            || ()
        });
    }

    let (pre_text, post_text) = game.text.split_at(game.index);
    html! {
        <div>
            <h1>{ "Typing Game" }</h1>
            <p>{ "Type the following word:" }</p>
            <span style="color:gray">{ pre_text }</span>{ post_text }
            <p>{ format!("Score: {}", game.score) }</p>
            <input
                type="text"
                onkeydown={on_keydown}
                style="opacity: 0; position: absolute;"
                ref={input_ref}
            />
        </div>
    }
}

#[wasm_bindgen]
pub fn run() {
    yew::Renderer::<App>::new().render();
}
