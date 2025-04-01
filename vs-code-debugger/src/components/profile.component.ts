import { execSync } from "child_process";
import { Component } from "../component";
import { Profile } from "../models/profile";

export class ProfileComponent extends Component {
    protected buildHtml(): string {
        const profiles: Profile[] = this.getProfiles();
        let profileOptions = [];

        for (let profile of profiles) {
            profileOptions.push(`
                <option
                    value="${profile.name}"
                    provider="${profile.llm}"
                    model="${profile.model}"
                    temperature="${profile.temperature}"
                    max-tokens="${profile.max_tokens}"
                >
                    ${profile.name} - ${profile.llm} (${profile.model})
                </option>
            `);
        }
    
        return `
            <div class="p-card">
                <h5>Profile</h5>
                <p class="p-card__content">
                    <select id="profile-select">
                        ${profileOptions}
                    </select>
                </p>
                <div class="disabled">
                    <div class="unfocused">Provider: <span id="profile-llm"></span></div>
                    <div class="unfocused">Model: <span id="profile-model"></span></div>
                    <div class="unfocused">Temperature: <span id="profile-temperature"></span></div>
                    <div class="unfocused">Max. tokens: <span id="profile-max-tokens"></span></div>
                </div>
            </div>
        `;
    }

    private getProfiles() {
        console.log("123");
        console.log(execSync('palang profiles --json'));
        console.log("456");
        return JSON.parse(execSync('palang profiles --json').toString().trim());
    }
}
