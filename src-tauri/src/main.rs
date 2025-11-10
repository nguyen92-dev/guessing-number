#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{self};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::io::Cursor;

#[derive(Clone)]
struct GameState(Arc<Mutex<InnerState>>);

impl Default for GameState {
    fn default() -> Self { Self(Arc::new(Mutex::new(InnerState::default()))) }
}

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

#[derive(Serialize, Deserialize)]
enum SoundKind { Correct, Wrong, Reset }

#[tauri::command]
fn play_sound(kind: SoundKind) -> Result<(), String> {
    let bytes: &'static [u8] = match kind {
        SoundKind::Correct => include_bytes!("../../src/sounds/correct.wav"),
        SoundKind::Wrong => include_bytes!("../../src/sounds/wrong.wav"),
        SoundKind::Reset => include_bytes!("../../src/sounds/reset.wav"),
    };
    std::thread::spawn(move || {
        if let Ok((stream, handle)) = rodio::OutputStream::try_default() {
            if let Ok(sink) = rodio::Sink::try_new(&handle) {
                if let Ok(decoder) = rodio::Decoder::new(Cursor::new(bytes)) {
                    sink.append(decoder);
                    sink.sleep_until_end();
                }
            }
            drop(stream);
        }
    });
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(GameState::default())
        .invoke_handler(tauri::generate_handler![start_game, guess_number, reset_game, play_sound])
        .run(tauri::generate_context!())
        .expect("error while running Tauri");
}
