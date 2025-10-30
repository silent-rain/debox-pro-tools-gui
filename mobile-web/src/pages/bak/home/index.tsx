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

const menus: MenuItem[] = [
  {
    title: 'AI绘画',
    icon: 'AI绘画',
    content: '',
    image: '/img/ai-painting.png',
    link: '/ai-painting',
  },
  {
    title: 'AI聊天',
    icon: 'AI绘画',
    content: '',
    image: '/img/ai-chat.png',
    link: '',
  },
  {
    title: 'AI搜索',
    icon: 'AI绘画',
    content: '',
    image: '/img/ai-search.png',
    link: '',
  },
  {
    title: 'AI语音',
    icon: 'AI绘画',
    content: '',
    image: '/img/ai-voice.png',
    link: '',
  },
];

interface MenuCardProps {
  menu: MenuItem;
}

const MenuCard = ({ menu: menu }: MenuCardProps) => {
  return (
    <Link className={styles.menu_card} to={menu.link}>
      <div className={styles.card_wrapper}>
        <Image src={menu.image} fit='cover' className={styles.image} />
        <h2 className={styles.title}>{menu.title}</h2>
      </div>
    </Link>
  );
};

export default function Home() {
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
