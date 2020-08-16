const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  mode: process.env.NODE_ENV || 'development',
  entry: './app/index.js',
  output: {
    path: path.resolve('static'),
    filename: 'index.js',
    publicPath: '/',
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
  ],
};
