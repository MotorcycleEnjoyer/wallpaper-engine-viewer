import { Outlet } from 'react-router-dom'
import Sidebar from '../Components/Sidebar'

export default function Root() {
  return (
    <div className="flex">
      <Sidebar />
      <Outlet />
    </div>
  )
}
