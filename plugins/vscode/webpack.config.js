//@ts-check

"use strict";

const path = require("path");
const fs = require("fs/promises");
const child_process = require("child_process");

//@ts-check
/** @typedef {import("webpack").Configuration} WebpackConfig **/
/** @typedef {import("webpack").Compiler} Compiler **/

/** @type WebpackConfig */
const extensionConfig = {
  target: "node", // VS Code extensions run in a Node.js-context 📖 -> https://webpack.js.org/configuration/node/
  mode: "none", // this leaves the source code as close as possible to the original (when packaging we set this to "production")

  entry: "./src/extension.ts", // the entry point of this extension, 📖 -> https://webpack.js.org/configuration/entry-context/
  output: {
    // the bundle is stored in the "dist" folder (check package.json), 📖 -> https://webpack.js.org/configuration/output/
    path: path.resolve(__dirname, "dist"),
    filename: "extension.js",
    libraryTarget: "commonjs2"
  },
  externals: {
    vscode: "commonjs vscode" // the vscode-module is created on-the-fly and must be excluded. Add other modules that cannot be webpack"ed, 📖 -> https://webpack.js.org/configuration/externals/
    // modules added here also need to be added in the .vscodeignore file
  },
  resolve: {
    // support reading TypeScript and JavaScript files, 📖 -> https://github.com/TypeStrong/ts-loader
    extensions: [".ts", ".js"]
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "ts-loader"
          }
        ]
      }
    ]
  },
  devtool: "nosources-source-map",
  infrastructureLogging: {
    level: "log", // enables logging required for problem matchers
  },
  plugins: [
    (compiler) => {
      compiler.hooks.beforeRun.tapPromise("CargoBuildPlugin", cargoBuild);
      compiler.hooks.watchRun.tapPromise("CargoBuildPlugin", cargoBuild);
    }
  ]
};
module.exports = [extensionConfig];

/**
 *
 * @param {string} command
 * @param {string[]} args
 * @param {child_process.SpawnOptions} options
 * @returns {Promise<void>}
 */
function spawnAsync(command, args, options) {
  return new Promise((resolve, reject) => {
    const childProcess = child_process.spawn(command, args, options);

    childProcess.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Command failed with code ${code}`));
      }
    });

    childProcess.on('error', (err) => {
      reject(err);
    });
  });
}

/**
 * @param {Compiler} compiler
 * @returns {Promise}
 */
async function cargoBuild(compiler) {
  const exeSuffix = process.platform == "win32" ? ".exe" : "";

  const isProduction = compiler.options.mode === "production";
  const cargo = process.env.CARGO || "cargo" + exeSuffix;
  const profile = isProduction ? "release" : "dev";
  const args = [
    "build",
    "--bin=wit-language-server",
    `--profile=${profile}`,
  ];
  if (process.env.CI) {
    args.push("--verbose");
  }
  const projectRoot = path.resolve(__dirname, "..", "..");

  console.log(`Executing: ${cargo} ${args.join(" ")}`);
  await spawnAsync(cargo, args, { cwd: projectRoot, stdio: "inherit" });

  await fs.mkdir(compiler.outputPath, { recursive: true });

  const binaryName = "wit-language-server" + exeSuffix;
  const targetDir = path.resolve(projectRoot, "target", isProduction ? "release" : "debug");
  const sourcePath = path.resolve(targetDir, binaryName);
  const destPath = path.resolve(compiler.outputPath, binaryName);

  await fs.copyFile(sourcePath, destPath);
  console.log(`Copied ${sourcePath} to ${destPath}`);
}
