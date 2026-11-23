<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import Selection from "./pages/Selection.svelte";
    import MainWindow from "./pages/MainWindow.svelte";

    let folderPath = $state('');

    onMount(async () => {
      folderPath = await invoke("get_cache_by_key", {k: "folder_path"})
    })
</script>

<main>
    {#if folderPath == ""}
        <Selection onPathChange={(p: string) => {folderPath = p}}/>
    {:else}
        <MainWindow folderPath={folderPath}/>
    {/if}
</main>
