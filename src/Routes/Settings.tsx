import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useEffect, useState } from 'react'

export default function Settings() {
  const [preferences, setPreferences] = useState({})

  async function handleClick() {
    const dir = await open({
      multiple: false,
      directory: true,
    })
    if (!dir) {
      return
    }

    const response = await invoke('set_wallpaper_directory', { dir })

    if (response) {
      console.log('The directory exists')
    } else {
      console.log('Directory does not exist.')
    }
    getPreferences()
  }

  async function getPreferences() {
    let preferences = await invoke('get_user_preferences')
    setPreferences(() => JSON.parse(preferences as string))
  }

  useEffect(() => {
    getPreferences()
  }, [])

  return (
    <div className="ml-auto mr-auto w-3/4 max-w-[500px]">
      {Object.entries(preferences).map(([key, value]) => (
        <div key={key} className="setting">
          <div className="rounded-2xl bg-black">
            <h1 className="overflow-clip p-2 text-xl font-bold">{key.split('_').join(' ')}</h1>
            <p className="overflow-clip p-2 text-center text-[#24c8db;]">{`${value}`}</p>
            {key === 'wallpaper_folder_location' && (
              <button className="w-full" onClick={handleClick}>
                Change Folder
              </button>
            )}
          </div>
        </div>
      ))}
    </div>
  )
}
