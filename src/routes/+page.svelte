<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import Selection from "./pages/Selection.svelte";

    let folderPath = $state("");

    onMount(async () => {
      folderPath = await invoke("get_cache", {key: "folder_path"})
    })
</script>

<main>
    {#if folderPath == ""}
        <Selection />
    {/if}
</main>
