import { exec, execSync } from 'child_process';
import path, { sep } from 'path';
import * as vscode from 'vscode';
import { ProfileComponent } from './components/profile.component';
import { WebviewPanel } from 'vscode';
import { ComponentsManager } from './component';
import { TaskComponent } from './components/task.component';
import { AssembliesComponent } from './components/assemblies.component';

export function activate(context: vscode.ExtensionContext) {
	const openDebuggerPaneDisposable = vscode.commands.registerCommand(
		'palang-debugger.open-debugger-pane',
		() => openDebuggerPane(context)
	);
	context.subscriptions.push(openDebuggerPaneDisposable);
}

export function deactivate() {}

function openDebuggerPane(context: vscode.ExtensionContext) {
	const panel = vscode.window.createWebviewPanel(
		'palang',
		'Palang',
		vscode.ViewColumn.Beside,
		{
			enableScripts: true,
		}
	);

	console.log(
		execSync(
			`palang profiles`
		).toString().trim()
	);

	panel.webview.onDidReceiveMessage(
		message => {
			switch (message.command) {
				case 'run':
					let args = message.arguments.map((arg: any) => `"${arg}"`).join(",");
					if (args !== "") {
						args = `--args ${args}`;
					}
					console.log(`palang run ${message.file} --task ${message.path} ${args} --profile ${message.profile}`);
					const response = execSync(
						`palang run ${message.file} --task ${message.path} ${args} --profile ${message.profile}`
					).toString().trim();
					switch (message.kind) {
						case 'prompt':
							panel.webview.postMessage({
								command: 'display-task-response',
								kind: 'prompt',
								path: message.path,
								text: response,
							});
							break;
						case 'function':
							panel.webview.postMessage({
								command: 'display-task-response',
								kind: 'function',
								path: message.path,
								text: response,
							});
							break;
					}
					break;
			}
		}
	);

	buildUI(context, panel).then(rootHtml => {
		panel.webview.html = rootHtml;
		ComponentsManager.update();

		const fileWatcher = vscode.workspace.onDidChangeTextDocument(
			(event: vscode.TextDocumentChangeEvent) => {
				const file = event.document.fileName;
				const extension = path.extname(file).toLowerCase();
				const workspace = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;

				if (workspace && path.dirname(file) === workspace && extension === '.palang') {
					ComponentsManager.update();
				}
			}
		);

		context.subscriptions.push(fileWatcher);
	});
}

async function buildUI(context: vscode.ExtensionContext, panel: WebviewPanel) {
	const assembliesView = buildAssembliesView(panel);

	const profilesView = buildProfilesView(panel);

	const isDark = vscode.window.activeColorTheme.kind === vscode.ColorThemeKind.Dark ? "class=\"is-dark\"" : "";

	const nonce = getNonce();
	const mainScript = panel.webview.asWebviewUri(vscode.Uri.joinPath(context.extensionUri, 'media', 'main.js'));

	return `
		<!DOCTYPE html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">
			<meta name=viewport" content="width=device-width, initial-scale=1.0">
			<title>Palang</title>
			<link rel="stylesheet" href="https://assets.ubuntu.com/v1/vanilla-framework-version-4.18.2.min.css" />
			<style>
				.unfocused {
					color: #999999;
				}

				.disabled {
					display: none;
				}

				nav {
					position: sticky;
					top: 0;
					padding-top: 20px;
					z-index: 100;
				}
			</style>
			<script>
				setInterval(() => {
					const profileSelect = document.getElementById('profile-select');
					const options = profileSelect.options[profileSelect.selectedIndex];

					const provider = options.getAttribute('provider');

					document.getElementById('profile-llm').innerHTML = provider;
				}, 500);
			</script>
			<script>
				window.addEventListener(
					'message',
					event => {
						const message = event.data;
						switch (message.command) {
							case 'update-component':
							{
								const element = document.getElementById(message.id);
								if (element) {
									element.innerHTML = message.content;
								}
								break;
							}
							case 'display-task-response':
							{
								const kind = message.kind;
								const path = message.path;
								const text = message.text;
								const safePath = path.replace(/\\//g, '-').toString();

								const responseElement = document.getElementById(\`\${kind}-response-\${path}\`);
								responseElement.innerHTML = text;
								responseElement.parentElement.classList.remove('disabled');

								const button = document.getElementById(\`btn-run-\${safePath}\`);
								button.innerHTML = 'Run';
								break;
							}
						}
					}
				);
			</script>
			<script nonce="${nonce}" src="${mainScript}" type="text/javascript" defer></script>
		</head>
		<body ${isDark}>
			<nav>
				${profilesView}
			</nav>
			<main>
				${assembliesView}
			</main>
		</body>
		</html>
	`;
}

function buildProfilesView(panel: WebviewPanel) {
	return new ProfileComponent("profiles", panel).getAnchor();
}

function buildAssembliesView(panel: WebviewPanel) {
	return new AssembliesComponent("assemblies", panel).getAnchor();
}

function getNonce() {
	let text = '';
	const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	for (let i = 0; i < 32; i++) {
		text += possible.charAt(Math.floor(Math.random() * possible.length));
	}
	return text;
}
