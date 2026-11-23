<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    let folderPath: String = $state(".")

    let list: {}[] | String = $state("")
</script>

<main>
    {#if folderPath == ""}
        <Selection />
    {/if}
</main>
