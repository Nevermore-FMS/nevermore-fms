const path = require("path");
var webpack = require('webpack');

module.exports = {
  entry: {
    worker: "./worker/index.ts",
    frontend: "./frontend/index.tsx",
  },
  devtool: "eval-source-map",
  externals: {
    react: "react",
  },
  externalsType: "window",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  plugins: [
    // apply this plugin only to .ts or .tsx files - the rest is taken care of
    new webpack.SourceMapDevToolPlugin({
      filename: null,
      exclude: [/node_modules/],
      test: /\.(ts|tsx)($|\?)/i,
    }),
  ],
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
  stats: "minimal",
};
