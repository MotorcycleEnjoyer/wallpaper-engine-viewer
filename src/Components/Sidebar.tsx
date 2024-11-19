import { NavLink } from 'react-router-dom'
import { useEffect, useState } from 'react'
import clsx from 'clsx'
import { invoke } from '@tauri-apps/api/core'

export default function Sidebar() {
  const [showSideBar, setShowSideBar] = useState(true)

  async function getPreferences() {
    let preferences = JSON.parse(await invoke('get_user_preferences'))
    setShowSideBar(preferences.is_sidebar_enabled)
  }

  useEffect(() => {
    // fetch stored user preference on sidebar
    getPreferences()
  }, [])

  function toggleView() {
    invoke('set_sidebar_status', { status: !showSideBar })
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
