import * as path from "path";
import * as fs from "fs";
import { ExtensionContext, ExtensionMode } from "vscode";
import { Executable, TransportKind } from "vscode-languageclient/node";
import { platform } from "process";

import { ServerConfig } from "./config";

const exeSuffix = platform == "win32" ? ".exe" : "";

/**
 * Load the configuration for starting the server executable.
 * @param context
 * @returns Executable settings
 */
export async function loadServerOptions(
    context: ExtensionContext,
    config: Partial<ServerConfig> | undefined,
): Promise<Executable> {
    const env = config?.extraEnv || {};

    switch (context.extensionMode) {
        case ExtensionMode.Production:
            let exePath = config?.path;

            if (!exePath) {
                exePath = path.resolve(
                    context.extensionPath,
                    "dist",
                    "wit" + exeSuffix,
                );
            }

            return {
                command: exePath,
                options: { env },
                args: ["language-server"],
                transport: TransportKind.stdio,
            };

        case ExtensionMode.Development:
        case ExtensionMode.Test:
            const transport = TransportKind.stdio;
            const options = {
                env: {
                    RUST_LOG: "debug,salsa_2022=warn",
                    RUST_BACKTRACE: "1",
                    PATH: process.env.PATH,
                    ...env,
                },
            };

            const cargoToml = await lspCargoToml(context);

            if (cargoToml) {
                return {
                    command: process.env["CARGO"] || "cargo",
                    args: [
                        "run",
                        "--quiet",
                        "--manifest-path",
                        cargoToml,
                        "--",
                        "language-server",
                    ],
                    transport,
                    options,
                };
            } else if (config?.path) {
                return { command: config.path, transport, options };
            } else {
                throw new Error(
                    `Unable to locate the project's root directory relative to "${context.extensionPath}" and the "wit-language-server.server.path" setting isn't set.`,
                );
            }
    }
}

async function lspCargoToml(
    context: ExtensionContext,
): Promise<string | undefined> {
    let dir = context.extensionPath;

    do {
        if (await exists(path.resolve(dir, ".git"))) {
            try {
                return path.resolve(
                    dir,
                    "crates",
                    "wit-cli",
                    "Cargo.toml",
                );
            } catch {
                continue;
            }
        }

        dir = path.dirname(dir);
    } while (dir && path.dirname(dir) != dir);
}

async function exists(path: string): Promise<boolean> {
    try {
        await fs.promises.access(path);
        return true;
    } catch {
        return false;
    }
}
