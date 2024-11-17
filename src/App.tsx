import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import './App.css'
import Home from './Routes/Home'
import Navbar from './Components/Navbar'

const router = createBrowserRouter([
  {
    path: '/',
    element: <Home />,
    children: [],
  },
])

function App() {
  return (
    <div className="flex">
      <Navbar />
      <RouterProvider router={router} />
    </div>
  )
}

export default App
