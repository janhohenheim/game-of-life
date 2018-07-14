const rust = import("../../../wasm_generated/game_of_life");
// rust.then(m => m.greet("World!"));


rust.then(rust => {
    const canvas = document.getElementById('game-board') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d');
    const game = rust.Game.new();
    game.start();
})
