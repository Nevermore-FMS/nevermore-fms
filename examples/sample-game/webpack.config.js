const path = require('path');

module.exports = {
  entry: {
      worker: './worker/index.ts',
      frontend: './frontend/index.tsx'
  },
  devtool: "eval-source-map",
  externals: {
    react: 'react',
  },
  externalsType: 'window',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
  stats: 'minimal'
};