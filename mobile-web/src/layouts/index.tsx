import { Suspense } from 'react';
import { Outlet, useLocation } from 'react-router-dom';

import Header from './header';
import Footer from './footer';
import styles from './index.module.less';

import { PageLoading } from '../components/page-loading';
import { useCurrentRouteMeta } from '@/routes/hooks/use-current-route-meta';

const Layout = () => {
  const matchedRouteMeta = useCurrentRouteMeta();
  const location = useLocation();
  console.log(location.pathname);

  return (
    <Suspense fallback={<PageLoading />}>
      <div className={styles.app}>
        {matchedRouteMeta ? <Header title={matchedRouteMeta.title} path={location.pathname} /> : <></>}

        <div className={styles.content}>
          <Outlet />
        </div>

        <Footer />
      </div>
    </Suspense>
  );
};

export default Layout;
