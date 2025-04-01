import { Model } from "./model";
import { Prompt } from "./prompt";
import { Function } from "./function";

export class Assembly {
    constructor(
        public name: string,
        public sourceFile: string,
        public models: Array<Model>,
        public prompts: Array<Prompt>,
        public functions: Array<Function>,
    ) {}
}
