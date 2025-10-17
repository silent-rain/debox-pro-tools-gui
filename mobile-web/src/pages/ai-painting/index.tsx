import { Grid, Image } from 'antd-mobile';
import styles from './index.module.less';
import { Link } from 'react-router-dom';

interface MenuItem {
  title: string;
  content: string;
  icon: string;
  image: string;
  link: string;
}

// 文生图风格
const menus: MenuItem[] = [
  {
    title: '文生图',
    icon: '',
    content: '',
    image: '/img/ai-painting.png',
    link: '/ai-painting/text-to-image-base',
  },
  {
    title: '图生图',
    icon: '',
    content: '',
    image: '/img/ai-chat.png',
    link: '',
  },
];

interface MenuCardProps {
  menu: MenuItem;
}

const MenuCard = ({ menu: menu }: MenuCardProps) => {
  return (
    <Link to={menu.link}>
      <div className={styles.card_wrapper}>
        <Image src={menu.image} fit='cover' className={styles.image} />
        <h2 className={styles.title}>{menu.title}</h2>
      </div>
    </Link>
  );
};

export default function AiPainting() {
  return (
    <>
      <div>
        <Grid columns={1} gap={10}>
          {menus.map((item) => (
            <Grid.Item key={item.title}>
              <MenuCard menu={item} />
            </Grid.Item>
          ))}
        </Grid>
      </div>
    </>
  );
}
