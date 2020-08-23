const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const mode = process.env.NODE_ENV || 'development';

module.exports = {
  mode,
  entry: ['./app/script/index.js', './app/style/index.css'],
  output: {
    path: path.resolve('static'),
    filename: 'index.js',
    publicPath: '/',
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
          'postcss-loader',
        ],
      },
    ],
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(),
      outDir: path.resolve('app', 'lib'),
      outName: 'index',
    }),
    new CopyPlugin({
      patterns: [
        { from: './app/index.html', to: './index.html' },
      ],
    }),
    new MiniCssExtractPlugin({ filename: 'index.css' }),
  ],
};
