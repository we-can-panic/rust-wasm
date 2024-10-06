use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, KeyboardEvent};

#[wasm_bindgen]
pub struct TypingGame {
    body: Document,
    text: String,
    index: usize,
    score: u32,
}

impl TypingGame {
    fn new_internal() -> Rc<RefCell<TypingGame>> {
        Rc::new(RefCell::new(TypingGame {
            body: {
                let window = web_sys::window().expect("no global `window` exists");
                window.document().expect("no global `document` exists")
            },
            text: "tekitou".to_string(),
            index: 0,
            score: 0,
        }))
    }

    // fn register_key_event_listener(game: Rc<RefCell<TypingGame>>) {
    //     let game_clone = Rc::clone(&game); // clone() で所有権を複製
    //     let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
    //         web_sys::console::log_1(&format!("You pressed: {}", event.key()).into());
            
    //         let mut game_ref = game_clone.borrow_mut();
    //         web_sys::console::log_1(&format!("text: {}\nindex: {}", game_ref.text, game_ref.index).into());

    //         if game_ref.text.chars().nth(game_ref.index).unwrap() == event.key().chars().nth(0).unwrap() {
    //         //     game_ref.index += 1;
    //         //     game_ref.display_text();
    //         web_sys::console::log_1(&"hhhh".to_string().into());
    //         }
    //     }) as Box<dyn FnMut(_)>);

    //     game.borrow().body.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    //     closure.forget();
    // }
    fn register_key_event_listener(game: Rc<RefCell<TypingGame>>) {
        let game_clone = Rc::clone(&game); // clone() で所有権を複製
        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            web_sys::console::log_1(&format!("You pressed: {}", event.key()).into());
            web_sys::console::log_1(&format!("text: {}\nindex: {}", game_clone.borrow().text, game_clone.borrow().index).into());
    
            if let Ok(mut game_ref) = game_clone.try_borrow_mut() {
                if game_ref.text.chars().nth(game_ref.index).unwrap() == event.key().chars().nth(0).unwrap() {
                    // web_sys::console::log_1(&format!("ref\ntext: {}\nindex: {}", game_ref.text, game_ref.index).into());
                    if game_ref.text.len() > game_ref.index + 1 {
                        game_ref.index += 1;
                    } else {
                        game_ref.index = 0;
                    }
                    game_ref.display_text();
                }
            } else {
                web_sys::console::log_1(&"Failed to borrow game_ref".into());
            }
        }) as Box<dyn FnMut(_)>);
    
        game.borrow().body.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
    

    fn display_text(&self) {
        let text_area = self.body.get_element_by_id("text-area").expect("no `text-area` here.");
        let html = format!(
            "<span style='color:gray'>{}</span>{}",
            &self.text[..self.index],
            &self.text[self.index..],
        );
        text_area.set_inner_html(&html);
    }
}

#[wasm_bindgen]
impl TypingGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TypingGame {
        TypingGame {
            body: {
                let window = web_sys::window().expect("no global `window` exists");
                window.document().expect("no global `document` exists")
            },
            text: "".to_string(),
            index: 0,
            score: 0,
        }
    }

    pub fn start(&mut self) {
        self.text = "tekitou".to_string();
        self.display_text();
        TypingGame::register_key_event_listener(TypingGame::new_internal());
        // TypingGame::register_key_event_listener(self);
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut game = TypingGame::new();
    game.start();
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

