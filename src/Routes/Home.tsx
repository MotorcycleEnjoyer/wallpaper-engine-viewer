import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
// when using `"withGlobalTauri": true`, you may use
// const { open } = window.__TAURI__.dialog;

export default function Home() {
  async function getDirectory() {
    // if no directory found, load the default view that says (Select Wallpaper Directory)
  }

  async function handleClick() {
    // Open a dialog
    const dir = await open({
      multiple: false,
      directory: true,
    })
    if (!dir) {
      return
    }

    const response = await invoke('store_wallpaper_directory', { dir })

    if (response) {
      console.log('The directory exists')
    } else {
      console.log('Directory does not exist.')
    }
    // TODO: Store this directory somewhere
  }

  return (
    <div className="p-4">
      <h1>Home</h1>
      <div className="flex justify-center">
        <button onClick={handleClick}>Select Wallpaper Directory</button>
        <button
          onClick={async () => {
            invoke('button')
          }}
        >
          CLICK ME
        </button>
      </div>
    </div>
  )
}
