import * as path from 'path';
import * as webpack from 'webpack';
import * as devServer from 'webpack-dev-server';
import HtmlWebpackPlugin = require('html-webpack-plugin');
// import { GenerateSW } from 'workbox-webpack-plugin';

const config: webpack.Configuration = {
    mode: 'development',
    devtool: 'inline-source-map',
    entry: './src/main.ts',
    module: {
      rules: [
          {
              test: /\.scss$/,
              use: [
                  {
                      loader: "style-loader"
                  }, {
                      loader: "css-loader"
                  }, {
                      loader: "sass-loader"
                  }
              ]
          },
          {
              test: /\.ts$/,
              use: [
                  'awesome-typescript-loader'
              ]
          },
          {
              test: /\.svg$/,
              use: ['svg-loader']
          },
          {
              test: /\.(eot|woff|woff2|svg|ttf)([\?]?.*)$/,
              use: ['file-loader']
          }
      ]
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'main.bundle.js',
    },
    devServer: {
        contentBase: path.join(__dirname, 'dist'),
        compress: true,
        port: 9000,
        writeToDisk: true // https://webpack.js.org/configuration/dev-server/#devserverwritetodisk-
    },
    plugins: [
        new HtmlWebpackPlugin({
            title: 'absQra'
        }),
        // new GenerateSW()
    ]
};

export default config;