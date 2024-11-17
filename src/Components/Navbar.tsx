import { useEffect, useState } from 'react'

export default function Navbar() {
  const [showSideBar, setShowSideBar] = useState(true)

  useEffect(() => {
    // fetch stored user preference on sidebar
  }, [])

  function toggleView() {
    setShowSideBar((prevState) => !prevState)
  }

  if (showSideBar) {
    return (
      <div className="flex min-h-[100vh] flex-col justify-center text-center">
        <button className="flex-1">Home</button>
        <button className="flex-1">Settings</button>
        <button className="flex-1">Stuff</button>
        <button className="flex-1" onClick={toggleView}>
          &lt;
        </button>
      </div>
    )
  } else {
    return (
      <div className="absolute bottom-0">
        <button className="flex-1 opacity-50 hover:opacity-100" onClick={toggleView}>
          &gt;
        </button>
      </div>
    )
  }
}
