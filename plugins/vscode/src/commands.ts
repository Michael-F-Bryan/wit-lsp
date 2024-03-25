import {
    ExtensionContext,
    TextDocumentContentProvider,
    Uri,
    ViewColumn,
    commands,
    window,
    workspace,
} from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import md from "markdown-it";

export function registerCommands(
    context: ExtensionContext,
    client: LanguageClient,
) {
    context.subscriptions.push(
        workspace.registerTextDocumentContentProvider(
            "wit",
            new WitAstProvider(client),
        ),
    );
    context.subscriptions.push(
        commands.registerCommand("wit-language-server.restart", async () => {
            if (client) {
                await client.restart();
                window.showInformationMessage("WIT Language Server restarted");
            }
        }),
    );
    context.subscriptions.push(
        commands.registerTextEditorCommand("wit-language-server.dump-ast", editor =>
            dumpAst(editor.document.uri),
        ),
    );
    context.subscriptions.push(
        commands.registerCommand(
            "wit-language-server.changelog",
            () => client && showChangelog(client),
        ),
    );
    context.subscriptions.push(
        commands.registerCommand(
            "wit-language-server.version",
            () => client && showVersion(client),
        ),
    );
}

async function showChangelog(client: LanguageClient) {
    const changelog = await client.sendRequest<string>("wit-language-server/changelog");
    if (!changelog) {
        window.showWarningMessage("No changelog available");
        return;
    }

    const panel = window.createWebviewPanel(
        "witChangelog",
        "Changelog",
        ViewColumn.Beside,
        {},
    );
    const rendered = md().render(changelog);
    panel.webview.html = `
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>WIT Language Server Changelog</title>
                </head>
                <body>
                    ${rendered}
                </body>
                </html>
            `;
}

/**
 * Parse a `*.wit` file and show its AST in a temporary window to the side.
 */
async function dumpAst(uri: Uri) {
    let witUri = Uri.from({
        scheme: "wit",
        path: "dump-ast",
        query: "path=" + encodeURI(uri.toString()),
    });

    let doc = await workspace.openTextDocument(witUri);

    await window.showTextDocument(doc, {
        preview: false,
        viewColumn: ViewColumn.Beside,
    });
}

async function showVersion(client: LanguageClient) {
    const serverInfo = client.initializeResult?.serverInfo;

    if (!serverInfo) {
        await window.showWarningMessage("Unable to show WIT server version: not initialized");
        return;
    }

    const version = serverInfo.version ? ` v${serverInfo.version}` : "";
    const notification = `${serverInfo.name}${version}`;
    await window.showInformationMessage(notification);
}

class WitAstProvider implements TextDocumentContentProvider {
    constructor(private readonly client: LanguageClient) { }

    async provideTextDocumentContent(uri: Uri): Promise<string> {
        switch (uri.path) {
            case "dump-ast":
                return await this.dumpAst(new URLSearchParams(uri.query));

            default:
                throw new Error(`Unknown method, ${uri.path}`);
        }
    }

    private async dumpAst(query: URLSearchParams) {
        const originalUri = query.get("path");

        if (!originalUri) {
            throw new Error('The "path" query parameter is missing');
        }

        const response = await this.client?.sendRequest<{ ast: string }>(
            "wit-language-server/dump-ast",
            { uri: decodeURI(originalUri) },
        );

        if (!response) {
            throw new Error("Missing response");
        }

        return response.ast;
    }
}
