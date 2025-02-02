import { WebviewPanel } from "vscode";

export abstract class Component {
    constructor(id: string, panel: WebviewPanel) {
        this.id = id;
        this.panel = panel;

        ComponentsManager.registerComponent(this);
    }

    getAnchor(): string { return `<span id="${this.id}"></span>`; }
    update(): void {
        this.panel.webview.postMessage({
            command: 'update-component',
            id: this.getId(),
            content: this.buildHtml()
        });
    }

    protected abstract buildHtml(): string;
    protected getId() { return this.id; }

    private id: string;
    private panel: WebviewPanel;
}

export class ComponentsManager {
    public static update(): void {
        for(let component of this.components) {
            component.update();
        }
    }

    public static registerComponent(component: Component): void {
        this.components.push(component);
    }

    private static components: Component[] = [];
    private static ids: Set<string> = new Set();
}
