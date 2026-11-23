<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";

    let path: string = $state("");
    const getSetPath = async () => {
      let a = await open({
        multiple: false,
        directory: true,
      })

      a == null ? path = path : path = a
      await invoke("set_cache", {k: "folder_path", v: path})
    }
</script>

<main class="w-screen h-screen flex flex-col justify-center items-center">
    <button onclick={() => {getSetPath()}} class="bg-gray-200 px-3 py-2 border">Select Music Folder</button>
    <h1>Selected path: {path}</h1>
</main>
