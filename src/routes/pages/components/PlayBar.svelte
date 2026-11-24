<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {Slider} from "bits-ui";

    let a: number = $state(50);
</script>
<main class="flex flex-row h-20 bg-main-highlight fixed bottom-0 justify-center items-center w-screen">

    <div class="flex flex-col relative">
        <div class="flex flex-row">
            <button onclick={async () => {
              await invoke("playback", {message: "PlayPause"})
            }} class="hover:underline">Play</button>
        </div>
        <div class="relative w-100 h-5">
            <div class="absolute w-full rounded-full h-full bg-green-500" > </div>
            <Slider.Root
                type="single"
                bind:value={a}
                class="relative flex w-full h-full touch-none items-center select-none"
              >
                <span
                  class="relative h-full w-full grow cursor-pointer overflow-hidden rounded-full bg-dark-10"
                >
                  <Slider.Range class="absolute h-full bg-foreground" />
                </span>
                <Slider.Thumb
                      index={0}
                      class={
                        "block size-1 invisible cursor-pointer rounded-full border border-border-input bg-background shadow-sm transition-colors hover:border-dark-40 focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 data-active:scale-[0.98] data-active:border-dark-40 dark:bg-foreground dark:shadow-card"
                      }
                    />
              </Slider.Root>
        </div>
        <h1>{a}</h1>
    </div>
</main>
