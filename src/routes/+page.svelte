<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  let prompt = ""
  let response = "Default output response text"
  let selectedButton = "Complete"

  async function complete() {
    response = await invoke("complete", { prompt })
  }

  async function edit() {
    response = await invoke("edit", { prompt })
  }

  async function clear() {
    prompt = ""
    response = ""
  }

</script>

<div class="input-container">
  <h1>OpenAI GUI</h1>
  <p>Selected button: {selectedButton}</p>
  <div class="row">
    <textarea bind:value={prompt} rows="5" cols="25"></textarea>
    <div class="column">
      <button on:click={complete} class="button">Complete</button>
      <button on:click={edit} class="button">Edit</button>
      <button class="button">Image</button>
      <button on:click={clear} class="button">Clear</button>
      <button class="button">Submit</button>
    </div>
  </div>
  <hr class="divider" />
  <div class="input-params">

    <div class="row">

      <!-- <label for="temperature">Temperature</label>
      <input type="number" id="temperature" name="temperature" min="0" max="1" step="0.1" value="0.5">
      <label for="credits">Maximum credits</label>
      <input type="number" id="credits" name="credits" min="0" max="1000" step="1" value="250"> -->
      <label for="temperature-slider">Temperature:</label>
      <input type="range" id="temperature-slider" name="temperature" min="0" max="1" step="0.1" value="0.5">

      <label for="credits-slider">Maximum credits:</label>
      <input type="range" id="credits-slider" name="credits" min="0" max="1000" step="1" value="250">

    </div>

  </div>
</div>
<hr class="divider" />
<div class="output-container">
  <h2>Output</h2>
  <p>{prompt}</p>
  <p>{response}</p>
</div>


<style>
  .input-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    /* height: 100vh; */
  }
  .output-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    /* height: 100vh; */
  }
  .divider {
    width: 100%;
  }
</style>