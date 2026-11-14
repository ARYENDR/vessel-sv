<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    let folderPath: String = $state(".")

    let list: {}[] | String = $state("")
</script>

<main>
    <button class="p-2" onclick={async () => {
      const a = await open({
        directory: true,
        multiple: false,
      })

      a == null ? null : folderPath = a
      list = await invoke("get_files", {path: folderPath})
      console.log(list)
    }}>Choose File</button>
    <h1>{folderPath}</h1>
    {#if typeof list != "string"}
        <ul>
            {#each list as entry}
                <li>{entry.title}</li>
            {/each}
        </ul>
    {:else}
        <h1>List is empty</h1>
    {/if}

</main>
