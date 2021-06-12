const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './app/index.ts',
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: "webassembly/sync",
            },
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.ts', '.tsx', '.js','.wasm']
    },
    output: {
        path: path.resolve(__dirname, 'public/dist/'),
        publicPath: "./dist/",
        filename: 'main.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './public/index.html'
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
            outDir: "public/pkg",
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        }),
    ],
    mode: 'development',
    experiments: {
        executeModule: true,
        outputModule: true,
        syncWebAssembly: true,
        topLevelAwait: true,
        asyncWebAssembly: true,
        layers: true,
    },
    devServer: {
        compress: true,
        publicPath: "/dist/",
        contentBase: path.join(__dirname, 'public'),
    },
};
