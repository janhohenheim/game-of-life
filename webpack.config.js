const path = require('path');

module.exports = {
    entry: "./app/client/game/game.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
    },
    mode: "development"
};