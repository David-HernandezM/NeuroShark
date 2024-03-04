import './app.scss';
import '@gear-js/vara-ui/dist/style.css';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { Routing } from './pages';
import { ApiLoader } from '@/components';
import { Header } from '@/components/layout';
import { withProviders } from '@/app/hocs';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import { 
  Root, 
  ErrorPage,
  Index,
  Account,
  XorDemoNN,
  DoodleDemoNN
} from './routes';


function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  const isAppReady = isApiReady && isAccountReady;

  const router = createBrowserRouter([
    {
      path: "/",
      element: <Root />,
      errorElement: <h1>ERROR A NIVEL ROOT</h1>,
      children: [
        {
          errorElement: <ErrorPage />,
          children: [
            {
              index: true,
              element: <Index />
            },
            {
              path: "account",
              element: <Account />,
            },
            {
              path: 'account/xor-demo-nn',
              element: <XorDemoNN />
            },
            {
              path: 'account/doodle-demo-nn',
              element: <DoodleDemoNN />
            }
          ]
        }
      ]
    },
    {
      path: "/*",
      element: <ErrorPage isRootError={true} />,
    }
  ]);

  return (
    <RouterProvider router={router} />
  );
}

export const App = withProviders(Component);
