import { WallpaperInfoType } from '../Mocks/wallpapers'
import preview from '../assets/preview.gif'

export default function WallpaperCard(props: WallpaperInfoType) {
  const MAX_TITLE_LENGTH = 20

  return (
    <div className="h-32 border border-transparent hover:cursor-pointer hover:border-[#24c8db;]">
      <div className="h-3/4 overflow-hidden bg-blue-500">
        <img src={preview} className="" alt="image" />
      </div>
      <div className="h-1/4 bg-black">
        <h1>
          {props.title.length > MAX_TITLE_LENGTH
            ? `${props.title.substring(0, MAX_TITLE_LENGTH)}...`
            : props.title}
        </h1>
      </div>
      {/* <h1>{title}</h1>
        <img src={previewFilePath} alt="previewImg" /> */}
      {/* <video src={previewFilePath}></video> */}
    </div>
  )
}
