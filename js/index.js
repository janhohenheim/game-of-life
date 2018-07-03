const rust = import("../wasm_generated/game_of_life");
rust.then(m => m.greet("World!"));
