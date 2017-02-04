let path = require('path');
let HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: './main.js',
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist')
    },
    module: {
        loaders: [
            {
                test: /\.js$/,
                exclude: /(node_modules|bower_components)/,
                loader: 'babel-loader',
                query: {
                    presets: ['es2015']
                }
            },
            {
                test: /\.scss$/,
                loaders: ["style-loader", "css-loader", "sass-loader"]
            }
        ]
    },
    plugins: [
        new HtmlWebpackPlugin({
            title: 'Infant Sleep Training Timer',
            filename: 'index.html',
            template: 'index.html'
        })
    ]
};