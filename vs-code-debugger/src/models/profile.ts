
export class Profile {
    constructor(
        name: string,
        llm: string,
        model: string,
        temperature: number,
        max_tokens: number
    ) {
        this.name = name;
        this.llm = llm;
        this.model = model;
        this.temperature = temperature;
        this.max_tokens = max_tokens;
    }

    public name: string;
    public llm: string;
    public model: string;
    public temperature: number;
    public max_tokens: number;
}
