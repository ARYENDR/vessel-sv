<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let {folderPath} = $props()

    let list: {}[] = $state([]);

    onMount(async () => {
      list = await invoke("get_audio_meta", {path: folderPath})
      console.log(folderPath)
      console.log(list)
    })

</script>

<main class="w-screen h-screen bg-main-bg text-white">
    <div class="flex flex-col p-5">
        {#each list as file}
            <button ondblclick={async () => {
              await invoke("playback", {message: "Play", path: file.path})
            }} class="text-start p-5 hover:bg-main-highlight focus:bg-main-active flex flex-row">
                <img class="h-10" src={convertFileSrc(file.path, "covers")} alt="">
                <div class="ml-2">
                    <h1>{file.title}</h1>
                    <div class="flex flex-row gap-3">
                        {#each file.artists as artist}
                            <!-- IMPL! artist pages -->
                            <a href={null} class="hover:underline">{artist}</a>
                        {/each}
                    </div>
                </div>
            </button>
        {/each}
    </div>
</main>
