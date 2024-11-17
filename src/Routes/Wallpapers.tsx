import { useEffect, useState } from 'react'
import WallpaperCard from '../Components/WallpaperCard'
import { dummyInfo, WallpaperInfoType } from '../Mocks/wallpapers'

export default function Wallpapers() {
  const [wallpapers, setWallpapers] = useState<WallpaperInfoType[] | []>([])
  useEffect(() => {
    setWallpapers(dummyInfo)
  }, [])
  return (
    <div className="flex-1">
      <div className="grid max-h-full w-full grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-8 2xl:grid-cols-10">
        {wallpapers.map((info) => (
          <WallpaperCard {...info} />
        ))}
      </div>
    </div>
  )
}
