<script lang="ts">
    import { open } from "@tauri-apps/plugin-dialog";

    let path: string = $state(".");
    const getSetPath = async () => {
      let a = await open({
        multiple: false,
        directory: true,
      })

      a == null ? path = "." : path = a
    }
</script>

<main class="w-screen h-screen flex flex-col justify-center items-center">
    <button class="bg-gray-200 px-3 py-2 border">Select Music Folder</button>
</main>
