import { NavLink } from 'react-router-dom'
import { useEffect, useState } from 'react'

export default function Sidebar() {
  const [showSideBar, setShowSideBar] = useState(true)

  useEffect(() => {
    // fetch stored user preference on sidebar
  }, [])

  function toggleView() {
    setShowSideBar((prevState) => !prevState)
  }

  return (
    <div>
      {showSideBar && (
        <div className="flex min-h-[100vh] flex-col text-center">
          <NavLink to="/" className="navlink">
            Home
          </NavLink>

          <NavLink to={'wallpapers'} className="navlink">
            WallPapers
          </NavLink>

          <NavLink to={'wallpapers'} className="navlink">
            WallPapers
          </NavLink>
        </div>
      )}
      <div className="absolute bottom-0">
        <button className="flex-1 opacity-50 hover:opacity-100" onClick={toggleView}>
          {showSideBar ? '<' : '>'}
        </button>
      </div>
    </div>
  )
}
