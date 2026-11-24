<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let isPlaying: boolean = false

</script>
<main class="flex flex-row h-20 bg-red-400 fixed bottom-0 justify-center items-center w-screen">

    <div class="flex flex-row gap-5">
        <button onclick={async () => {
          await invoke("playback", {message: "Seek", duration: -10})
        }} class="hover:underline">Prev 10s</button>

        <button onclick={async () => {
          await invoke("playback", {message: "PlayPause"})
        }} class="hover:underline">Play</button>

        <button onclick={async () => {
          await invoke("playback", {message: "Seek", duration: 10})
        }} class="hover:underline">Seek 10</button>    </div>

</main>
