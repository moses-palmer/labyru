const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: path.resolve(__dirname, 'index.js'),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        /*new HtmlWebpackPlugin({
            template: path.resolve(__dirname, 'index.html'),
        }),*/
    ],
    devServer: {
        host: '0.0.0.0',
        port: 8080,
    },
    mode: 'development'
};
