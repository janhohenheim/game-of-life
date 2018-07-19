const rust = import('../../../wasm_generated/game_of_life')

rust.then(rust => {
    const canvas = document.getElementById('game-board') as HTMLCanvasElement
    const game = rust.EntryPoint.new(canvas)

    canvas.addEventListener('click', (e) => {
        const pos = getMousePos(canvas, e)
        game.on_click(pos.x, pos.y)
    })

    const startStop = document.getElementById('start-stop') as HTMLInputElement
    const speedSlider = document.getElementById('speed-slider') as HTMLInputElement

    let timerId = 0
    let speed = Number(speedSlider.value)
    let isGameRunning = false

    startStop.addEventListener('click', (e) => {
        if (isGameRunning) {
            stopGame()
            startStop.innerText = "Start"
        } else {
            setGameSpeed(speed)
            startStop.innerText = "Stop"
        }
        isGameRunning = !isGameRunning
    })

    speedSlider.addEventListener('change', (e) => {
        speed = Number(speedSlider.value)
        setGameSpeed(speed)
    })

    function stopGame() {
        window.clearInterval(timerId)
    }

    function setGameSpeed(speed: number) {
        stopGame()
        const timeout = Number(speedSlider.max) - speed
        timerId = window.setInterval(() => {
            game.on_timer()
        }, timeout)
    }
})

class MousePosition {
    x: number
    y: number
}

function getMousePos(canvas: HTMLCanvasElement, e: MouseEvent): MousePosition {
    const rect = canvas.getBoundingClientRect()
    const scaleX = canvas.width / rect.width
    const scaleY = canvas.height / rect.height
    const x = (e.clientX - rect.left) * scaleX
    const y = (e.clientY - rect.top) * scaleY

    return {
        x: Math.round(x),
        y: Math.round(y)
    }
}
