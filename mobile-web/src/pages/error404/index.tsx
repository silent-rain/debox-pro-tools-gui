import { Button, ErrorBlock } from 'antd-mobile';
import { useNavigate } from 'react-router';
import { ROUTES } from '@/constants/routes';
import styles from './index.module.css';

export default function Error404() {
  const navigate = useNavigate();

  const click = () => {
    navigate(ROUTES.HOME);
  };

  return (
    <div className={styles.container}>
      <ErrorBlock status='default' />
      <Button className={styles.button} color='primary' fill='solid' onClick={click}>
        Return to Home
      </Button>
    </div>
  );
}
