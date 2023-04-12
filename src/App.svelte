<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api'
  import { listen } from '@tauri-apps/api/event'

  type RenamedFile = {
    old_path: string
    old_name: string
    new_path: string
    new_name: string
  }

  let paths: string[] = []
  let renamedFiles: RenamedFile[] = []

  onMount(async () => {
    listen<string[]>('tauri://file-drop', async (event) => {
      paths = [...event.payload]
      invoke<RenamedFile[]>('handle_drag_and_drop_files', { paths })
        .then((res) => {
          renamedFiles = res
        })
        .catch((err) => {
          console.error(err)
          return []
        })
    })
  })

  function handleRename() {
    invoke('handle_rename_files', {
      renamedFiles: renamedFiles,
    }).catch((err) => {
      console.error(err)
      return []
    })
  }

  function handleCancel() {
    renamedFiles = []
  }
</script>

{#if !renamedFiles.length}
  <div class="drop-area">ファイルをここにドロップしてください</div>
{/if}

{#if renamedFiles.length}
  <ul>
    {#each renamedFiles as file}
      <li>{file.old_name} : {file.new_name}</li>
    {/each}
  </ul>
  <button on:click={handleRename}>リネームする</button>
  <button on:click={handleCancel}>キャンセルする</button>
{/if}

<style>
  .drop-area {
    border: 2px dashed #888;
    padding: 20px;
    text-align: center;
    margin: 20px;
  }
</style>
