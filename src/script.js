window.addEventListener('DOMContentLoaded', async () => {
    if (!window.__TAURI__) {
        console.error('Tauri API not found! Make sure you\'re running in Tauri app.');
        return;
    }

    const {invoke} = window.__TAURI__.core;

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

    async function play(kind) {
        try {
            await invoke('play_sound', { kind });
        } catch (e) {
            console.warn('play_sound failed', e);
        }
    }

    document.getElementById('guess-btn').addEventListener('click', async () => {
        const guess = parseInt(document.getElementById('guess').value);
        if (isNaN(guess)) return;
        gameState = await invoke('guess_number', {guess});
        updateUI();
        if (gameState.game_over) {
            play('Correct');
        } else {
            play('Wrong');
        }
        document.getElementById('guess').value = '';
    });

    document.getElementById('reset-btn').addEventListener('click', async () => {
        gameState = await invoke('reset_game');
        updateUI();
        play('Reset');
    });

    document.getElementById('exit-btn').addEventListener('click', () => {
        window.__TAURI__.window.getCurrentWindow().close();
    });

    await initGame();

})