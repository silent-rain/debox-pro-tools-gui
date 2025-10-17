import { NavBar } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import styles from './header.module.less';

interface HeaderProps {
  title: string;
}

// 头部导航栏
const Header = ({ title }: HeaderProps) => {
  const navigate = useNavigate();

  return (
    <NavBar
      className={styles.header}
      onBack={() => {
        navigate(-1);
      }}
    >
      {title}
    </NavBar>
  );
};

export default Header;
