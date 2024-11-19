import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useEffect } from 'react'

export default function Home() {
  useEffect(() => {
    invoke('first_time_setup')
  })

  async function handleClick() {
    const dir = await open({
      multiple: false,
      directory: true,
    })
    if (!dir) {
      return
    }
    await invoke('set_wallpaper_directory', { dir })
  }

  return (
    <div className="w-full p-4">
      <h1 className="p-4 text-3xl">Wallpaper Engine Viewer</h1>
      <div className="flex justify-center">
        <button onClick={handleClick}>Select Wallpaper Directory</button>
      </div>
    </div>
  )
}
