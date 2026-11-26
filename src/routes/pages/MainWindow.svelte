<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    // import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    let { folderPath } = $props()

    let list: {}[] = $state([]);

//     let currentlyPlaying: {} | null = $state(
//       listen("currently_playing", (c) => {return c})
//     )
// 
    onMount(async () => {
      list = await invoke("get_audio_meta", {path: folderPath})
    })

</script>

<main class="w-screen h-screen bg-main-bg text-white overflow-x-scroll">
    <div class="flex flex-col p-5">
        {#each list as file}
            <button ondblclick={async () => {
              await invoke("playback", {message: "Play", path: file.path})}} class="text-start p-2 outline-none rounded-md hover:bg-main-highlight focus:bg-main-active flex flex-row items-center justify-start">
                <img loading="lazy" class="h-10 rounded-md" src={convertFileSrc(file.path, "covers")} alt="">
                <div class="ml-2">
                    <h1>{file.title}</h1>
                    <div class="flex flex-row gap-1">
                        {#each file.artists as artist, i}
                            <!-- IMPL! artist pages -->

                            {#if file.artists.length > 1 && file.artists.length -1 != i}
                                    <a href={null} class="hover:underline flex flex-row">{artist},</a>
                            {:else}
                                <a href={null} class="hover:underline">{artist}</a>
                            {/if}

                        {/each}
                    </div>
                </div>
            </button>
        {/each}
    </div>
</main>
