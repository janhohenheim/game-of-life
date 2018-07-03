const rust = import("./wasm_generated/game");
rust.then(m => m.greet("World!"));