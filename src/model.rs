extern crate js_sys;

#[derive(Clone)]
pub struct TypingGame {
    word_list: Vec<String>,
    word_idx: usize,
    pub text: String,
    pub index: usize,
    pub score: u32,
}

impl TypingGame {
    pub fn new() -> Self {
        let mut result = TypingGame {
            word_list: shuffle(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string(), "grape".to_string(), "grape2".to_string()]),
            word_idx: 0,
            text: "".to_string(),
            index: 0,
            score: 0,
        };
        result.text = result.word_list[result.word_idx].clone();
        result

    }

    pub fn on_key_input(&self, key: char) -> Self{
        // web_sys::console::log_1(&format!("You pressed: {}", key).into());
        let mut new_state = self.clone();
        if self.text.chars().nth(self.index) == Some(key) {
            if self.text.len() > self.index + 1 {
                new_state.index += 1;
                new_state.score += 1;
            } else {
                new_state.next_word();
            }
        }
        new_state
    }

    fn next_word(&mut self) {
        self.word_idx = if self.word_idx + 1 < self.word_list.len() {
            self.word_idx + 1
        } else {
            0
        };
        self.text = self.word_list[self.word_idx].clone();
        self.index = 0;
    }
}   

pub fn shuffle(array: Vec<String>) -> Vec<String>{
    let mut array = array.clone();
    let len = array.len();
    for i in 0..len {
        // 配列の末尾からランダムなインデックスを選択
        let j = (js_sys::Math::random() * (i as f64)).floor() as usize;
        // 要素を交換
        array.swap(i, j);
    }
    array
}
