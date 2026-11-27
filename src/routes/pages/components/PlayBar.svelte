<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let currentlyPlaying = $state({})
    let stamp: number = $state(0)

    let sliderStamp = $derived(stamp);
    let newStamp = $state(0)
    let isPlaying = $state(false)

    $effect(() => {

      const cpnlp = listen<{}>("currently_playing", (c) => {
        currentlyPlaying = c.payload
        // console.log(c.payload)
        typeof currentlyPlaying.length != "number" ? currentlyPlaying.length = 0 : currentlyPlaying.length
      })

      const ispl = listen<boolean>("is_playing", (i) => {
        isPlaying = i.payload
      })

      const sunlp = listen<number>('stamp', (s) => {
          stamp = s.payload;
          // console.log(stamp)
      });

      return () => {
        cpnlp.then((unlisten) => unlisten());
        sunlp.then((unlisten) => unlisten());
      };
    });

    // $effect(() => {
    //   const unlp = listen<number>('currently_playing', (s) => {
    //     console.log(s.payload);
    //       currentlyPlaying = s.payload;
    //   });
    //   return () => {
    //     unlp.then((unlisten) => unlisten());
    //   };
    // });

    import svgPlay from "../../../lib/assets/play.svg"
    import svgPause from "../../../lib/assets/pause.svg"
    import svgNext from "../../../lib/assets/next.svg"
    import svgPrev from "../../../lib/assets/previous.svg"
    import svgSeekback from "../../../lib/assets/seek-backwards.svg"
    import svgSeekforw from "../../../lib/assets/seek-forward.svg"

</script>
<main class="flex flex-row h-25 bg-main-highlight py-10 fixed bottom-0 justify-around items-center w-screen dark:text-white">

    <div class="flex flex-row ml-5 w-100 items-center gap-5">
        <img class="max-h-20 rounded-md" src={convertFileSrc(currentlyPlaying.path, "covers")} alt="">
        <div class="overflow-hidden">
            <!-- TODO: marquee animation, at least -->
            <h1 class="overflow-scroll scrollbar-none whitespace-nowrap auto-cols-max">{currentlyPlaying.title}</h1>
            <div class="flex flex-row scrollbar-none gap-1 whitespace-nowrap overflow-scroll">
                {#each currentlyPlaying.artists as artist, i}

                    {#if currentlyPlaying.artists.length > 1 && currentlyPlaying.artists.length -1 != i}
                        <h1 class="text-sm">{artist}, </h1>
                    {:else}
                        <h1 class="text-sm">{artist}</h1>
                    {/if}
                {/each}
            </div>
        </div>
    </div>
    <div class="flex flex-col relative gap-3 justify-center mx-auto self-center">
        <div class="flex flex-row justify-center items-items-center h-fit gap-5">
            <button>
                <img class="max-h-6 dark:invert" src={svgPrev} alt="">
            </button>
            <button onclick={async () => {
              await invoke("playback", {message: "PlayPause"})
            }} class="hover:underline">
                <img class="max-h-10 dark:invert hover:scale-105" src={isPlaying ? svgPause : svgPlay} alt="">
            </button>
            <button>
                <img class="max-h-6 dark:invert" src={svgNext} alt="">
            </button>
        </div>
        <div class="relative w-full h-5">
            <div class="rounded-full w-150 bg-main-fg h-2 hover:h4 relative overflow-hidden">
                <div style:width="{((sliderStamp/currentlyPlaying.length) * 100).toString()}%" class="absolute w-1 h-full bg-green-500"></div>


                <!-- BUG: This works but only half the time -->
                <input onchange={
                    async () => {await invoke("playback", {message: "Set", duration: sliderStamp}); sliderStamp = stamp}
                    } bind:value={sliderStamp} onclick={() => {sliderStamp = newStamp}} class="appearance-none bg-transparent accent-amber-400 opacity-0 outline-none bg-none cursor-pointer w-full" type="range" min={0} max={currentlyPlaying.length}>
            </div>
            <div class="flex flex-row justify-between dark:text-white">
                <h1>{Math.floor(stamp)}</h1>
                <h1>{Math.floor(currentlyPlaying.length)}</h1>
            </div>
        </div>
    </div>
    <div class="w-100 mr-5 flex flex-row justify-end">
        <div class="w-50 flex flex-row justify-between">
            <button>
                L
            </button>
            <button>
                W
            </button>
            <button>
                S
            </button>
            <button>
                V
            </button>
        </div>
    </div>
</main>
