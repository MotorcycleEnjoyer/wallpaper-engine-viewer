import { invoke } from '@tauri-apps/api/core'
import { WallpaperInfoType } from '../Mocks/wallpapers'
import { join } from '@tauri-apps/api/path'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useState } from 'react'

export default function WallpaperCard({
  info,
  wallpaperDirectory,
}: {
  info: WallpaperInfoType
  wallpaperDirectory: string
}) {
  const MAX_TITLE_LENGTH = 20
  const [imageSrc, setImageSrc] = useState('')

  async function getFilePath() {
    const filePath = await join(wallpaperDirectory, info.project_id, info.project_json.preview)
    const assetUrl = convertFileSrc(filePath)
    console.log(assetUrl)
    setImageSrc(assetUrl)
  }
  getFilePath()

  function openVideo() {
    console.log('calling play_video')
    const videoPath = `${wallpaperDirectory}\\${info.project_id}\\${info.project_json.file}`
    invoke('play_video', { videoPath })
  }

  return (
    <div className="h-32 border border-transparent hover:border-[#24c8db;]" onClick={openVideo}>
      <div
        className="flex h-full flex-col justify-end overflow-hidden bg-cover bg-center bg-no-repeat"
        style={{
          backgroundImage: `url(${imageSrc})`,
        }}
      >
        <h1 className="bg-black bg-opacity-50 p-1">
          {info.project_json.title.length > MAX_TITLE_LENGTH
            ? `${info.project_json.title.substring(0, MAX_TITLE_LENGTH)}...`
            : info.project_json.title}
        </h1>
      </div>
    </div>
  )
}
