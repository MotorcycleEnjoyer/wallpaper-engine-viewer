export default function WallpaperCard({
  title,
  filePath,
  previewFilePath,
}: {
  title: string
  filePath: string
  previewFilePath: string
}) {
  const MAX_TITLE_LENGTH = 20

  return (
    <div className="h-32 border border-transparent hover:cursor-pointer hover:border-[#24c8db;]">
      <div className="h-3/4 bg-blue-500">
        <img src="" alt="image" />
      </div>
      <div className="h-1/4 bg-black">
        <h1>
          {title.length > MAX_TITLE_LENGTH ? `${title.substring(0, MAX_TITLE_LENGTH)}...` : title}
        </h1>
      </div>
      {/* <h1>{title}</h1>
        <img src={previewFilePath} alt="previewImg" /> */}
      {/* <video src={previewFilePath}></video> */}
    </div>
  )
}
