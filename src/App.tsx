import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "./App.css";

const router = createBrowserRouter([
  {
    path: "/",
    element: <h1>Hello World!</h1>,
    children: [],
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
