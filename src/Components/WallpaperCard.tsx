import { WallpaperInfoType } from '../Mocks/wallpapers'
import preview from '../assets/preview.gif'

export default function WallpaperCard(props: WallpaperInfoType) {
  const MAX_TITLE_LENGTH = 20

  return (
    <div className="h-32 border border-transparent hover:border-[#24c8db;]">
      <div
        className="flex h-full flex-col justify-end overflow-hidden bg-cover bg-center bg-no-repeat"
        style={{ backgroundImage: `url(${preview})` }}
      >
        <h1 className="bg-black bg-opacity-50 p-1">
          {props.title.length > MAX_TITLE_LENGTH
            ? `${props.title.substring(0, MAX_TITLE_LENGTH)}...`
            : props.title}
        </h1>
      </div>
    </div>
  )
}
