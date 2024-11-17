import { NavLink } from 'react-router-dom'
import { useEffect, useState } from 'react'
import clsx from 'clsx'

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
      <div
        className={clsx(
          showSideBar
            ? 'flex min-h-[100vh] flex-col bg-[#0f0f0f] text-center'
            : 'hidden transition-all',
        )}
      >
        <NavLink to="/" className="navlink">
          Home
        </NavLink>

        <NavLink to="wallpapers" className="navlink">
          WallPapers
        </NavLink>

        <NavLink to="settings" className="navlink">
          Settings
        </NavLink>
      </div>
      <div className="fixed bottom-0">
        <button className="flex-1 opacity-50 hover:opacity-100" onClick={toggleView}>
          {showSideBar ? '<' : '>'}
        </button>
      </div>
    </div>
  )
}
