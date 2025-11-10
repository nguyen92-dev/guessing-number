#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{self, Manager};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Default, Clone, Serialize, Deserialize)]
struct GameState(Arc<Mutex<InnerState>>);

#[derive(Default, Clone, Serialize, Deserialize)]
struct InnerState {
    secret_number: u32,
    message: String,
    game_over: bool,
}

#[tauri::command]
fn start_game(state: tauri::State<GameState>) -> InnerState {
    let mut inner = state.0.lock().unwrap();
    let mut rng = rand::rng();
    *inner = InnerState {
        secret_number: rng.random_range(1..=100),
        message: "Chào bé! Đoán số từ 1 đến 100 nhé! 😊".to_string(),
        game_over: false,
    };
    inner.clone()
}

#[tauri::command]
fn guess_number(state: tauri::State<GameState>, guess: u32) -> InnerState {
    let mut inner = state.0.lock().unwrap();
    if guess < 1 || guess > 100 {
        inner.message = "Số phải từ 1 đến 100 thôi bé ơi! 😅".to_string();
    } else if guess == inner.secret_number {
        inner.message = format!("Chính xác! Bé giỏi quá! 🎊 Số đúng là {}.", inner.secret_number);
        inner.game_over = true;
    } else if guess > inner.secret_number {
        inner.message = "Số bé đoán lớn quá rồi! Thử nhỏ hơn nhé! 📉".to_string();
    } else {
        inner.message = "Số bé đoán hơi bé rồi! Thử lớn hơn đi! 📈".to_string();
    }
    inner.clone()
}

#[tauri::command]
fn reset_game(state: tauri::State<GameState>) -> InnerState {
    let mut inner = state.0.lock().unwrap();
    let mut rng = rand::rng();
    *inner = InnerState {
        secret_number: rng.random_range(1..=100),
        message: "Chơi lại nhé! Đoán số mới đi! 🎲".to_string(),
        game_over: false,
    };
    inner.clone()
}

fn main() {
    tauri::Builder::default()
        .manage(GameState(Default::default()))
        .invoke_handler(tauri::generate_handler![start_game, guess_number, reset_game])
        .run(tauri::generate_context!())
        .expect("error while running Tauri");
}
