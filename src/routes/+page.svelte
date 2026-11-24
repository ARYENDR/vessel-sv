<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import Selection from "./pages/Selection.svelte";
    import MainWindow from "./pages/MainWindow.svelte";
    import PlayBar from "./pages/components/PlayBar.svelte";

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
        <PlayBar />
    {/if}
</main>
