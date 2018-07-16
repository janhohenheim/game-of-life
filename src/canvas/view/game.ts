const rust = import("../../../wasm_generated/game_of_life");

rust.then(rust => {
    const canvas = document.getElementById('game-board') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d');
    const game = rust.Game.new();

    canvas.addEventListener('click', (e) => {
        const pos = getMousePos(canvas, e);
        console.log(pos);
    });
    game.start();
})

class MousePosition {
    x: number;
    y: number;
}

function getMousePos(canvas: HTMLCanvasElement, e: MouseEvent): MousePosition {
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    const x = (e.clientX - rect.left) * scaleX;
    const y = (e.clientY - rect.top) * scaleY;

    return {
        x: Math.round(x),
        y: Math.round(y)
    }
}