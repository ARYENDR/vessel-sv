<script lang="ts">
    import { open } from "@tauri-apps/plugin-dialog";
    let folderPath: String = $state("")
</script>

<main>
    <button class="p-2" onclick={async () => {
      const a = await open({
        directory: true,
        multiple: false,
      })

      a == null ? null : folderPath = a
    }}>Choose File</button>
    <h1>{folderPath}</h1>
</main>
