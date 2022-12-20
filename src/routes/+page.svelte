<script lang='ts'>
import { invoke } from '@tauri-apps/api/tauri'
import PromptCard from '$lib/PromptCard.svelte';
import ResponseCard from '$lib/ResponseCard.svelte';
import { json } from '@sveltejs/kit';

let model = "GPT3";
let temperature = 0.7;
let maxCredits = 250;
let prompt = "";

// let prompts: string[] = ["1", "2", "3"];
// let responses: string[] = ["1.5", "2.5", "3.5"];

let messages = [
{
    type: "response",
    text: "Send me a message!"
}
];

// Create a clear function that resets messages and prompt
function clear() {
    messages = [];
    prompt = "";
}

async function send() {
    console.log("Sending message...");
    messages.push({
        type: "prompt",
        text: prompt
    });
    messages = messages;
    let response = await invoke("complete", { prompt, model, temperature, maxCredits });
    console.log(response);
    // Strip any new line characters at the beginning of the response
    messages.push({
        type: "response",
        text: JSON.stringify(response)
    });
    prompt = "";
    messages = messages;
}

function printMessages() {
    console.log(messages);
}

</script>

<div class="grid-container">
    <nav>
        <h1>OpenAI GUI</h1>
    </nav>
    <main>
        {#each messages as message}
            {#if message.type === "prompt"}
                <PromptCard text={message.text} />
            {:else}
                <ResponseCard text={message.text} />
            {/if}
        {/each}
    </main>
    <div id="sidebar">
        <!-- Create a dropdown selector with a label called Model: -->
        <label for="model-select">Model: {model}</label>
        <select bind:value={model} name="model-select" id="model-select">
            <option value="GPT3">GPT3</option>
            <option value="davinci">Davinci</option>
            <option value="curie">Curie</option>
            <option value="babbage">Babbage</option>
            <option value="ada">Ada</option>
        </select>
        <!-- Create a slider with a label called Temperature: -->
        <label for="temperature-slider">Temperature: {temperature}</label>
        <input bind:value={temperature} type="range" id="temperature-slider" name="temperature" min="0" max="1" step="0.1">
        <!-- Create a slider with a label called Maximum credits: -->
        <label for="credits-slider">Maximum credits: {maxCredits}</label>
        <input bind:value={maxCredits} type="range" id="credits-slider" name="credits" min="0" max="1000" step="1">
        <!-- Create a button with the text Send -->
        <button on:click={send} class="button">Send</button>
        <!-- Create a button with the text Clear -->
        <button on:click={clear} class="button">Clear</button>
        <button on:click={printMessages} class="button">Log Messages</button>

    </div>
    <footer>
        <!-- Create an input text box for the chat box application. The footer should contain a text input box that grows taller to accommodate overflow text -->
        <!-- Then add a send button to the right of the text box inline -->
        <input bind:value={prompt} type="text" placeholder="Type message here...">
        <button on:click={send} type="submit">Send</button>
    </footer>
</div>

<style>
.grid-container {
    display: grid;
    height: 95vh;
    grid-template-columns: 3fr 1fr;
    grid-template-rows: 1fr 10fr 1fr;
    grid-template-areas:
        "nav nav"
        "main sidebar"
        "footer footer";
    grid-gap: 0.2rem;
}
nav {
    /* background: #a7ffeb; */
    border-radius: 10px;
    border: 2px solid black;
    grid-area: nav;
}
main {
    /* background: #f48fb1; */
    border-radius: 10px;
    border: 2px solid black;
    grid-area: main;
    overflow: auto;
    display: flex;
    flex-direction: column;
}
#sidebar {
    /* background: #b39ddb; */
    border-radius: 10px;
    border: 2px solid black;
    grid-area: sidebar;
    display: flex;
    flex-direction: column;
}
footer {
    /* background: #b2ebf2; */
    border-radius: 10px;
    border: 2px solid black;
    grid-area: footer;

    display: flex;
    flex-direction: row;
}
input {
    flex-grow: 1;
}

@media only screen and (max-width: 600px) {
    .grid-container {
        grid-template-columns: 1fr;
        grid-template-rows: 1fr 10fr 1fr 1fr;
        grid-template-areas:
            "nav"
            "main"
            "footer"
            "sidebar";
    }
}
</style>