<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = "";
  let greetMsg = "";

  async function greet() {
    setTimeout(async () => {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      greetMsg = await invoke("greet", { name });
    }, 1000);
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
