import { execSync } from "child_process";
import { Component } from "../component";
import { Profile } from "../models/profile";
import { WebviewPanel } from "vscode";

export class TaskComponent extends Component {
    constructor(id: string, panel: WebviewPanel, assembly: any) {
        super(id, panel);
        this.assembly = assembly;
    }

    private assembly: any;

    protected buildHtml(): string {
        let prompts = "";
        for (let prompt of this.toSortedArray(this.assembly.prompts, "name")) {
            prompts += this.buildPromptView(prompt);
        }

        let functions = "";
        for (let func of this.toSortedArray(this.assembly.functions, "name")) {
            functions += this.buildFunctionView(func);
        }

        return `
            <div>
                Assembly <i class="p-icon--chevron-right"></i> ${this.formatAssemblyName(this.assembly.name)}
                ${prompts}
                ${functions}
            </div>
        `;
    }

    private buildPromptView(prompt: any) {
        let parameters = "";
        for (let parameter of prompt.parameters) {
            parameters += this.buildParameter(parameter);
        }
    
        const id = `run-prompt-${prompt.name}`;
    
        return `
            <div class="p-card">
                <h5>${this.formatAssemblyName(prompt.name)}</h5>
                <p class="p-card__content">
                    ${parameters}
                    <button onclick="runTask('prompt', '${prompt.name}')">Run</button>
                </p>
            </div>
        `;
    }

    private buildFunctionView(func: any) {
        let parameters = "";
        for (let parameter of func.parameters) {
            parameters += this.buildParameter(parameter);
        }
    
        const id = `run-function-${func.name}`;
    
        return `
            <div class="p-card">
                <h5>${this.formatAssemblyName(func.name)}</h5>
                <p class="p-card__content">
                    ${parameters}
                    <button onclick="runTask('function', '${func.name}')">Run</button>
                </p>
            </div>
        `;
    }
    
    private buildParameter(parameter: any) {
        return `
            <label>${this.toUpperCamelCase(parameter.name)}</label>
            <input type="text"></input>
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

    private getProfiles() {
        return JSON.parse(execSync('palang profiles --json').toString().trim());
    }
}
