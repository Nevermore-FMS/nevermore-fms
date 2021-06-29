const path = require('path');

module.exports = {
  entry: {
    worker: './src/worker.ts',
    frontend: './src/frontend.tsx'
  },
  externals: {
    react: 'React'
  },
  externalsType: 'window',
  target: 'web',
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
};