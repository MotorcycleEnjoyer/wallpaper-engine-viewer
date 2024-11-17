import { open } from "@tauri-apps/plugin-dialog";
// when using `"withGlobalTauri": true`, you may use
// const { open } = window.__TAURI__.dialog;

export default function Home() {
  async function getDirectory() {
    // if no directory found, load the default view that says (Select Wallpaper Directory)
  }

  async function handleClick() {
    // Open a dialog
    const file = await open({
      multiple: false,
      directory: true,
    });
    console.log(file);
    // TODO: Store this directory somewhere
  }

  return (
    <div>
      <h1>Home</h1>
      <div className="flex justify-center">
        <button onClick={handleClick}>Select Wallpaper Directory</button>
      </div>
    </div>
  );
}
