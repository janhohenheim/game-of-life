const rust = import("../wasm_generated/game_of_life");
// rust.then(m => m.greet("World!"));


rust.then(module => {
    const canvas = document.getElementById('game-board') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d');
    module.init_board(ctx);
})
