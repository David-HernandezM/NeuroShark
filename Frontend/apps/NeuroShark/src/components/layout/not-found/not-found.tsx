import styles from './not-found.module.scss';
import { Link } from 'react-router-dom';
import { buttonVariants } from '@/components/ui/button/button';
import { Heading } from '@/components/ui/heading';
import { ROUTES } from '@/app/consts';
import ImageWebp from './assets/images/404.webp';
import ImageBase from './assets/images/404.jpg';

export function NotFound() {
  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.header}>
          <Heading size="lg" style={{ fontSize: '110px', color: 'white', fontWeight: 'bold' }}>
            Oops!
          </Heading>
          <p className={styles.message404}>404 - PAGE NOT FOUND</p>
          <p className={styles.message404Description}>
            The page you are looking for might have been removed, had its name changed or is temporarily unavailable
          </p>
        </div>
        <Link to={ROUTES.HOME} className={buttonVariants({ variant: 'white' })} style={{}}>
          Back To Home
        </Link>
      </div>
    </div>
  );
}
