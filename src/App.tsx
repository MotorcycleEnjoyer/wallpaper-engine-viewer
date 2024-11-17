import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import './App.css'
import Home from './Routes/Home'
import Navbar from './Components/Navbar'
import Wallpapers from './Routes/Wallpapers'
import Root from './Routes/Root'
import Settings from './Routes/Settings'

const router = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
    children: [
      {
        path: '/',
        element: <Home />,
        errorElement: <h1>Failed to load Home route</h1>,
      },
      {
        path: 'wallpapers',
        element: <Wallpapers />,
        children: [],
        errorElement: <h1>Failed to load Wallpapers route</h1>,
      },
      {
        path: 'settings',
        element: <Settings />,
        children: [],
        errorElement: <h1>Failed to load Settings route</h1>,
      },
    ],
    errorElement: <h1>Failed to load root route</h1>,
  },
])

function App() {
  return (
    <div className="flex">
      <RouterProvider router={router} />
    </div>
  )
}

export default App
