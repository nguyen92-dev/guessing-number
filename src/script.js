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

    function safePlay(id) {
        const el = document.getElementById(id);
        if (!el) return;
        // Lazy load sources if needed
        if (el.preload === 'none' && el.readyState === 0) {
            // Trigger load
            el.load();
        }
        const playPromise = el.play();
        if (playPromise && typeof playPromise.then === 'function') {
            playPromise.catch(err => {
                console.warn('Audio play failed:', id, err.message);
            });
        }
    }

    document.getElementById('guess-btn').addEventListener('click', async () => {
        const guess = parseInt(document.getElementById('guess').value);
        if (isNaN(guess)) return;
        gameState = await invoke('guess_number', {guess});
        updateUI();
        if (gameState.game_over) {
            safePlay('correct-sound'); // Âm thanh đúng
        } else {
            safePlay('wrong-sound'); // Âm thanh sai
        }
        document.getElementById('guess').value = '';
    });

    document.getElementById('reset-btn').addEventListener('click', async () => {
        gameState = await invoke('reset_game');
        updateUI();
        safePlay('reset-sound'); // Âm thanh reset
    });

    document.getElementById('exit-btn').addEventListener('click', () => {
        window.__TAURI__.window.getCurrentWindow().close();
    });

    await initGame();

})