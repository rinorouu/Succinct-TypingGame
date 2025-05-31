use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::{console, HtmlInputElement, Document, Element, KeyboardEvent};

#[derive(Serialize, Deserialize)]
pub struct GameResult {
    pub player_name: String,
    pub score: u32,
    pub proof: String,
}

#[wasm_bindgen]
pub struct TypingGame {
    words: Vec<String>,
    current_word: String,
    score: u32,
    player_name: String,
    started: bool,
}

#[wasm_bindgen]
impl TypingGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let words = vec![
            "rust", "wasm", "zk", "blockchain", "crypto",
            "proof", "speed", "typing", "test", "game",
            "leaderboard", "score", "validate", "player",
            "input", "name", "start", "finish", "timer"
        ].iter().map(|s| s.to_string()).collect();

        TypingGame {
            words,
            current_word: String::new(),
            score: 0,
            player_name: String::new(),
            started: false,
        }
    }

    #[wasm_bindgen]
    pub fn set_player_name(&mut self, name: String) {
        self.player_name = name;
    }

    #[wasm_bindgen]
    pub fn start_game(&mut self) {
        self.started = true;
        self.score = 0;
        self.pick_new_word();
    }

    #[wasm_bindgen]
    pub fn handle_input(&mut self, input: String) -> bool {
        if !self.started {
            return false;
        }

        if input.trim() == self.current_word {
            self.score += 1;
            self.pick_new_word();
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn get_current_word(&self) -> String {
        self.current_word.clone()
    }

    #[wasm_bindgen]
    pub fn get_score(&self) -> u32 {
        self.score
    }

    #[wasm_bindgen]
    pub fn is_started(&self) -> bool {
        self.started
    }

    #[wasm_bindgen]
    pub fn generate_result(&self) -> JsValue {
        let proof = generate_proof(self.player_name.clone(), self.score);
        let result = GameResult {
            player_name: self.player_name.clone(),
            score: self.score,
            proof,
        };
        JsValue::from_serde(&result).unwrap()
    }

    fn pick_new_word(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.words.len());
        self.current_word = self.words[idx].clone();
    }
}

fn generate_proof(player_name: String, score: u32) -> String {
    // In a real implementation, this would generate an actual zk proof
    // using SP1 SDK. This is a simplified version for demonstration.
    format!("proof-{}-{}", player_name, score)
}
