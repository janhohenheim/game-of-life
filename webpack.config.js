const path = require('path');

module.exports = {
    entry: "./app/client/game/game.js",
    node: false,
    output: {
        publicPath: '/dist/',
        path: path.resolve(__dirname, "dist"),
        filename: "[name].js",
    },
    resolve: {
        extensions: ['.ts', '.js', '.wasm'],
    },
    mode: "development",
};