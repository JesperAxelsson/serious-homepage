const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const HtmlWebPackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
// const ExtractTextPlugin = require('extract-text-webpack-plugin')

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    experiments: {
      asyncWebAssembly: true,
    },
    entry: './bootstrap.js',
    devtool: 'eval-source-map',
    devServer: {
      static: distPath,

      // allowedHosts: path.join(__dirname, 'dist'),
      compress: argv.mode === 'production',
      port: 8000
    },
    output: {
      path: distPath,
      filename: "homepage.js",
      webassemblyModuleFilename: "homepage.wasm"
    },
    module: {
      rules: [
        // {
        //   test: /\.css$/,
        //   use: ExtractTextPlugin.extract({
        //     fallback: 'style-loader',
        //     use: [
        //       { loader: 'css-loader', options: { importLoaders: 1 } },
        //       'postcss-loader',
        //     ],
        //   }),
        // },
        // {
        //   test: /\.css$/,
        //   use: [
        //     MiniCssExtractPlugin.loader,
        //     "css-loader", "postcss-loader",
        //   ],
        // }
      ]
    },
    plugins: [
      new HtmlWebPackPlugin({
        template: "src/index.html",
        filename: "index.html"
      }),
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath }
        ]
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
