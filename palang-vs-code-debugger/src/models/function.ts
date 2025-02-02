import { Parameter } from "./parameter";

export class Function {
    constructor(
        public name: string,
        public parameters: Array<Parameter>,
        public return_type: string,
        public instructions: Array<any>,
    ) {}
}
