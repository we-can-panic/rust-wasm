use core::borrow;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, KeyboardEvent};

#[wasm_bindgen]
pub struct TypingGame {
    body: Document,
    text: String,
    index: usize,
    score: u32,
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

    fn display_text(&self) {
        let text_area = self.body.get_element_by_id("text-area").expect("no `text-area` here.");
        let html = format!(
            "<span style='color:gray'>{}</span>{}",
            &self.text[..self.index],
            &self.text[self.index..],
        );
        text_area.set_inner_html(&html);
    }

    pub fn start(&mut self) {
        self.text = "tekitou".to_string();
        self.display_text();
        TypingGame::register_key_event_listener(TypingGame::new_internal());
    }
}

// Rustの借用に関する処理はJSにエクスポートできないので内部関数で作成
/*
借用について
* 借用 = 参照渡し
* 借用した変数の元の持ち主を「所有者」と呼ぶ。所有者は基本変わらない。
* 借用は以下のルールがある
    1. 可変借用は1つのみ存在可能
    2. 不変借用（のみ）であれば複数作成可能
* 拡張として、Rc, RefCellなどの操作がある（所有権の共有）
    * RefCell: Reference Cell
        * 基本は不変借用
        * borrow_mut() を呼び出すとき、一時的に可変借用として持ち出せる
            * このとき、可変借用不可であれば実行時エラーとなる
    * Rc: Reference counted
        * 参照を渡すと複数参照可能になる
    * Rc, RefCellともにクラス内でアクセスを制御しているので、&による複数参照とは微妙に動作が違う。
*/
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
    fn register_key_event_listener(game: Rc<RefCell<TypingGame>>) {
        // clone() を使って借用を増やす。元の借用はイベントリスナーで可変借用として使う。
        let clone = Rc::clone(&game);

        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // web_sys::console::log_1(&format!("You pressed: {}", event.key()).into());
            // web_sys::console::log_1(&format!("text: {}\nindex: {}", game.borrow().text, game.borrow().index).into());
            if let Ok(mut game_ref) = game.try_borrow_mut() {
                // 可変参照をして、値を操作
                if game_ref.text.chars().nth(game_ref.index).unwrap() == event.key().chars().nth(0).unwrap() {
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

        clone.borrow().body.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();

        closure.forget();
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut game = TypingGame::new();
    game.start();
}
