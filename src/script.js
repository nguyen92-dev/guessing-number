const { invoke } = window.__TAURI__.tauri;

let gameState;

async function initGame() {
  gameState = await invoke('start_game');
  updateUI();
}

function updateUI() {
  document.getElementById('message').textContent = gameState.message;
  if (gameState.game_over) {
    document.getElementById('guess').disabled = true;
    document.getElementById('guess-btn').disabled = true;
  } else {
    document.getElementById('guess').disabled = false;
    document.getElementById('guess-btn').disabled = false;
  }
}

document.getElementById('guess-btn').addEventListener('click', async () => {
  const guess = parseInt(document.getElementById('guess').value);
  if (isNaN(guess)) return;
  gameState = await invoke('guess_number', { guess });
  updateUI();
  if (gameState.game_over) {
    document.getElementById('correct-sound').play(); // Âm thanh đúng
  } else {
    document.getElementById('wrong-sound').play(); // Âm thanh sai
  }
  document.getElementById('guess').value = '';
});

document.getElementById('reset-btn').addEventListener('click', async () => {
  gameState = await invoke('reset_game');
  updateUI();
  document.getElementById('reset-sound').play(); // Âm thanh reset
});

document.getElementById('exit-btn').addEventListener('click', () => {
  window.close();
});

initGame();
