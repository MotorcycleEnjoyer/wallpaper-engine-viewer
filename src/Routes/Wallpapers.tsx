import { useEffect, useState } from 'react'
import WallpaperCard from '../Components/WallpaperCard'
import { dummyInfo, WallpaperInfoType } from '../Mocks/wallpapers'
import { invoke } from '@tauri-apps/api/core'

export default function Wallpapers() {
  const [wallpapers, setWallpapers] = useState<WallpaperInfoType[] | []>([])
  const [loading, setLoading] = useState(false)

  async function fetch_all_wallpapers() {
    setLoading(true)
    let info = await invoke('get_all_wallpapers')

    // minimum delay of 250ms
    setTimeout(() => {
      setLoading(false)
    }, 250)

    console.log(info)
  }

  useEffect(() => {
    setWallpapers(dummyInfo)
    fetch_all_wallpapers()
  }, [])

  if (loading) {
    // MIT License for the loading SVG
    return (
      // Copyright 2020 Vjacheslav Trushkin
      // Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
      // The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
      // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
      <div className="flex flex-1 justify-center">
        <h1 className="m-auto p-8 text-2xl text-cyan-500">
          <svg xmlns="http://www.w3.org/2000/svg" width="5rem" height="5rem" viewBox="0 0 24 24">
            <g
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
            >
              <path stroke-dasharray="16" stroke-dashoffset="16" d="M12 3c4.97 0 9 4.03 9 9">
                <animate fill="freeze" attributeName="stroke-dashoffset" dur="0.3s" values="16;0" />
                <animateTransform
                  attributeName="transform"
                  dur="1.5s"
                  repeatCount="indefinite"
                  type="rotate"
                  values="0 12 12;360 12 12"
                />
              </path>
              <path
                stroke-dasharray="64"
                stroke-dashoffset="64"
                stroke-opacity="0.3"
                d="M12 3c4.97 0 9 4.03 9 9c0 4.97 -4.03 9 -9 9c-4.97 0 -9 -4.03 -9 -9c0 -4.97 4.03 -9 9 -9Z"
              >
                <animate fill="freeze" attributeName="stroke-dashoffset" dur="1.2s" values="64;0" />
              </path>
            </g>
          </svg>
        </h1>
      </div>
    )
  }

  return (
    <div className="flex-1">
      <div className="grid w-full grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-8 2xl:grid-cols-10">
        {wallpapers.map((info) => (
          <WallpaperCard {...info} />
        ))}
      </div>
    </div>
  )
}
