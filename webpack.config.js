const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const isProd = process.env.NODE_ENV === 'production';

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
            {
                test: /\.s[ac]ss$/i,
                use: ['style-loader', 'css-loader', 'less-loader'],
            },
            {
                test: /\.(woff(2)?|ttf|eot|svg)(\?v=\d+\.\d+\.\d+)?$/,
                use: [
                    {
                        loader: 'file-loader',
                        options: {
                            name: '[name].[ext]',
                            outputPath: 'fonts/'
                        }
                    }
                ]
            }
        ],
    },
    resolve: {
        extensions: ['.ts', '.tsx', '.js','.wasm']
    },
    output: {
        path: path.resolve(__dirname, 'public/dist/'),
        // Use an absolute public path to avoid double "dist" when resolving dynamic chunks.
        publicPath: isProd
            ? '/a-puzzle-a-day-solver/dist/'
            : '/dist/',
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
        outputModule: true,
        syncWebAssembly: true,
        topLevelAwait: true,
        asyncWebAssembly: true,
        layers: true,
    },
    devServer: {
        host: "127.0.0.1",
        port: 8081,
        compress: true,
        devMiddleware : {
            publicPath: "/dist/",
        },
        static: {
            directory: path.join(__dirname, "/public"),
        },
    },
};
