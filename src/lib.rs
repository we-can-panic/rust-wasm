mod model;

use model::{TypingGame, WordList};
use yew::prelude::*;
use yew::events::KeyboardEvent;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;

#[function_component(App)]
fn app() -> Html {
    let game = use_state(|| None::<TypingGame>);
    let input_ref = use_node_ref();

    let fetch_words = {
        let game = game.clone();

        use_effect_with((),
            move |_| {
                spawn_local(async move {
                    let fetched: WordList = Request::get("/words.json")
                        .send()
                        .await
                        .expect("Failed to fetch words")
                        .json()
                        .await
                        .expect("Failed to parse JSON");

                    game.set(Some(TypingGame::new(fetched.words)));
                });
                || ()
            },
        );
    };

    let on_keydown = {
        let game = game.clone();
        Callback::from(move |e: KeyboardEvent| {
            if let Some(game_ref) = game.as_ref() {
                let key = e.key().chars().nth(0);
                if let Some(c) = key {
                    game.set(Some(game_ref.on_key_input(c)));
                    // game.on_key_input(c);
                }
            }
        })
    };

    use_effect({
        let input_ref = input_ref.clone();
        move || {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                input.focus().ok();
            }
            || ()
        }
    });

    html! {
        <div>
            <h1>{ "Typing Game" }</h1>
            {
                if let Some(game) = game.as_ref() {
                    let (pre_text, post_text) = game.text.split_at(game.index);
                    html! {
                        <>
                            <p>{ "Type the following word:" }</p>
                            <span style="color:gray">{ pre_text }</span>{ post_text }
                            <p>{ format!("Score: {}", game.score) }</p>
                        </>
                    }
                } else {
                    html! { <p>{ "Loading..." }</p> }
                }
            }
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
