import { ExtensionContext, ExtensionMode, commands, window, workspace } from "vscode";
import {
	LanguageClient,
	LanguageClientOptions,
} from "vscode-languageclient/node";
import { registerCommands } from "./commands";
import { ServerConfig } from "./config";
import { loadServerOptions } from "./server-options";

let client: LanguageClient | undefined;

export async function activate(context: ExtensionContext) {
	const config = workspace.getConfiguration("wit-language-server");
	const serverConfig: Partial<ServerConfig> | undefined = config.get("server");

	let serverOptions = await loadServerOptions(context, serverConfig);

	if (!serverOptions) {
		// No binary available
		await window.showWarningMessage(
			`No language server binary available. Set "wit-language-server.server.path" to a "wit-language-server" executable.`,
		);
		return;
	}

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "wit" }],
	};

	// Create the language client and start the client.
	client = new LanguageClient(
		"WitLanguageServer",
		"WIT Language Server",
		serverOptions,
		clientOptions,
	);
	registerCommands(context, client);

	await client.start();

	if (context.extensionMode != ExtensionMode.Production) {
		await commands.executeCommand('workbench.action.toggleDevTools');
	}
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
