import styles from './index.module.css';

export default function Error404() {
  return (
    <div className={styles.container}>
      <h1 className={styles.title}>404</h1>
      <p className={styles.message}>Oops! The page you're looking for doesn't exist.</p>
      <button className={styles.button} onClick={() => (window.location.href = '/')}>
        Return to Home
      </button>
    </div>
  );
}
