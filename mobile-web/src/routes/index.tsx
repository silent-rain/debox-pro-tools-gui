import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { RootRoutes } from './routes';

export default function Router() {
  const router = createBrowserRouter(RootRoutes());

  return <RouterProvider router={router} />;
}
