import { Parameter } from "./parameter";

export class Prompt {
    constructor(
        public name: string,
        public parameters: Array<Parameter>,
        public return_type: string,
        public text: string,
    ) {}
}
