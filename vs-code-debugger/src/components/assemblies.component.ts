import { execSync } from "child_process";
import { Component } from "../component";
import { Profile } from "../models/profile";
import * as vscode from 'vscode';
import { WebviewPanel } from "vscode";
import { Assembly } from "../models/assembly";
import * as path from 'path';
import * as fs from 'fs';

export class AssembliesComponent extends Component {
    constructor(id: string, panel: WebviewPanel) {
        super(id, panel);
        let ref = this;
    }

    protected buildHtml(): string {
        let view = "";

        const assemblies = describeAssembliesSync();
        for (let assembly of assemblies.sort((a, b) => -b.name.localeCompare(a.name))) {
            assembly.models = Object.values(assembly.models).sort((a: any, b: any) => -b.name.localeCompare(a.name));
            assembly.prompts = Object.values(assembly.prompts).sort((a: any, b: any) => -b.name.localeCompare(a.name));
            assembly.functions = Object.values(assembly.functions).sort((a: any, b: any) => -b.name.localeCompare(a.name));
            view += this.buildAssemblyView(assembly);
        }

        return view;
    }

    private buildAssemblyView(assembly: Assembly) {
        let prompts = "";
        for (let prompt of this.toSortedArray(assembly.prompts, "name")) {
            prompts += this.buildPromptView(prompt, assembly.sourceFile);
        }

        let functions = "";
        for (let func of this.toSortedArray(assembly.functions, "name")) {
            functions += this.buildFunctionView(func, assembly.sourceFile);
        }

        return `
            <div>
                Assembly <i class="p-icon--chevron-right"></i> ${this.formatAssemblyName(assembly.name)}
                ${prompts}
                ${functions}
            </div>
        `;
    }

    private buildPromptView(prompt: any, sourceFile: string) {
        let parameters = "";
        for (let parameter of prompt.parameters) {
            if (parameter.name === '') {
                continue;
            }
            parameters += this.buildParameter(prompt.name, parameter);
        }

        const safeName = prompt.name.replace(/\//g, '-').toString();

        return `
            <div class="p-card">
                <h5>(Prompt) ${this.formatAssemblyName(prompt.name)}</h5>
                <p class="p-card__content">
                    ${parameters}
                    <button id="btn-run-${safeName}" onclick="runTask('prompt', '${prompt.name}', '${sourceFile}')">Run</button>
                    <pre class="disabled">
                        <div id="prompt-response-${prompt.name}"></div>
                    </pre>
                </p>
            </div>
        `;
    }

    private buildFunctionView(func: any, sourceFile: string) {
        let parameters = "";
        for (let parameter of func.parameters) {
            parameters += this.buildParameter(func.name, parameter);
        }

        const safeName = func.name.replace(/\//g, '-').toString();

        return `
            <div class="p-card">
                <h5>(Function) ${this.formatAssemblyName(func.name)}</h5>
                <p class="p-card__content">
                    ${parameters}
                    <button id="btn-run-${safeName}" onclick="runTask('function', '${func.name}', '${sourceFile}')">Run</button>
                    <pre class="disabled">
                        <div id="function-response-${func.name}"></div>
                    </pre>
                </p>
            </div>
        `;
    }
    
    private buildParameter(taskName: string, parameter: any) {
        const safeName = taskName.replace(/\//g, '-').toString();
        return `
            <label>${this.toUpperCamelCase(parameter.name)}</label>
            <input type="text" class="parameter-${safeName}"></input>
        `;
    }
    
    private formatAssemblyName(fullyQualifiedName: String): String {
        const separator = fullyQualifiedName.lastIndexOf("/") + 1;
        const path = fullyQualifiedName.substring(0, separator).replaceAll("/", "::");
        const name = fullyQualifiedName.substring(separator);
    
        return `<span class="unfocused">${path}</span>${name}`;
    }
    
    private toSortedArray(object: Object, sortKey: any) {
        return Object.values(object).sort((a, b) => a[sortKey] - b[sortKey]);
    }
    
    private toUpperCamelCase(text: String): String {
        return text.replace(/(?:^\w|[A-Z]|\b\w|\s+)/g, function(match, index) {
            if (+match === 0) {
                return "";
            }
            return match.toUpperCase();
        });
    }

}

// async function describeAssemblies() {
//     const files = await vscode.workspace.findFiles('**/*', '**/node_modules/**');
//     let fileDescriptions = [];

//     for (const file of files) {
//         let description = JSON.parse(execSync(`palang describe ${file.path} --json`).toString().trim());
//         description.sourceFile = file.path;
//         fileDescriptions.push(description);
//     }

//     return fileDescriptions;
// }

function describeAssembliesSync(): any[] {
    const workspaceFolders = vscode.workspace.workspaceFolders;

    if (!workspaceFolders) {
        console.log("No workspace folders found.");
        return [];
    }

    let fileDescriptions: any[] = [];

    // Traverse each workspace folder
    workspaceFolders.forEach(folder => {
        const folderPath = folder.uri.fsPath;

        // Recursively find files in the folder, excluding node_modules
        const files = findFilesSync(folderPath, '**/node_modules/**');

        files.forEach(file => {
            try {
                // Execute the command synchronously and parse the output
                const commandOutput = execSync(`palang describe ${file} --json`).toString().trim();
                let description = JSON.parse(commandOutput);
                description.sourceFile = file;
                fileDescriptions.push(description);
            } catch (error) {
                console.error(`Error processing file: ${file}`, error);
            }
        });
    });

    return fileDescriptions;
}

function findFilesSync(dir: string, excludePattern: string): string[] {
    let results: string[] = [];
    const items = fs.readdirSync(dir);

    items.forEach(item => {
        const fullPath = path.join(dir, item);
        const stat = fs.statSync(fullPath);

        if (stat.isDirectory()) {
            // Skip excluded directories like node_modules
            if (!fullPath.includes(excludePattern)) {
                results = results.concat(findFilesSync(fullPath, excludePattern));
            }
        } else if (stat.isFile()) {
            results.push(fullPath);
        }
    });

    return results;
}
