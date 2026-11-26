<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { Slider } from "bits-ui";

    let currentlyPlaying = $state({})
    let stamp: number = $state(0)

    $effect(() => {

      const cpnlp = listen<{}>("currently_playing", (c) => {
        currentlyPlaying = c.payload
        console.log(c.payload)
      })

      const sunlp = listen<number>('stamp', (s) => {
          stamp = s.payload;
          console.log(stamp)
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


</script>
<main class="flex flex-row h-20 bg-main-highlight fixed bottom-0 justify-center items-center w-screen">

    <div class="flex flex-col relative">
        <div class="flex flex-row">
            <button onclick={async () => {
              await invoke("playback", {message: "PlayPause"})
            }} class="hover:underline">Play</button>
        </div>
        <div class="relative w-full h-5">
            <div class="rounded-full w-150 bg-main-fg h-2 hover:h4 relative overflow-hidden">
                <div class="absolute w-{sliderStamp} h-full bg-green-500"></div>

                <!-- BUG: This works but only half the time -->
                <input onchange={
                    async () => {await invoke("playback", {message: "Set", duration: sliderStamp}); sliderStamp = stamp;}
                    } bind:value={sliderStamp} onmousedown={() => sliderStamp = newStamp} class="cursor-pointer w-full" type="range" min={0} max={currentlyPlaying.length}>
            </div>
        </div>
        <h1>Slider: {stamp} - {Math.floor(currentlyPlaying.length)} - {sliderStamp}</h1>
    </div>
</main>
