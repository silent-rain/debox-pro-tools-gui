import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card, List, Button, Toast } from 'antd-mobile';
import { InformationCircleOutline, RightOutline, GlobalOutline, MailOutline, FileOutline } from 'antd-mobile-icons';
import styles from './index.module.less';

const About = () => {
  const navigate = useNavigate();
  const [appInfo] = useState({
    version: '1.0.0',
    buildTime: '2024-12-01',
    appName: 'DeBox Pro Tools',
    description: '专业的DeBox账号管理和群组工具箱',
  });

  const features = [
    '账号管理：支持多账号管理，快速切换',
    '群组同步：一键同步所有群组信息',
    '成员管理：批量管理群组成员',
    '关注互助：智能关注管理，提升账号影响力',
    '批量聊天：高效的消息群发功能',
    '一键拉群：快速创建和管理群组',
  ];

  const handleCopyVersion = () => {
    navigator.clipboard.writeText(appInfo.version);
    Toast.show('版本号已复制');
  };

  const handleContact = (type: string) => {
    switch (type) {
      case 'email':
        Toast.show('邮箱：silentrains2020@gmail.com');
        break;
      case 'website':
        Toast.show('官网：暂无');
        break;
      case 'phone':
        Toast.show('客服热线：暂无');
        break;
      default:
        break;
    }
  };

  return (
    <div className={styles.about}>
      {/* 应用信息卡片 */}
      <Card className={styles.appCard}>
        <div className={styles.appHeader}>
          <div className={styles.appIcon}>
            <InformationCircleOutline />
          </div>
          <div className={styles.appInfo}>
            <h2>{appInfo.appName}</h2>
            <p className={styles.version} onClick={handleCopyVersion}>
              版本 {appInfo.version}
            </p>
            <p className={styles.buildTime}>构建时间：{appInfo.buildTime}</p>
          </div>
        </div>
        <p className={styles.description}>{appInfo.description}</p>
      </Card>

      {/* 功能介绍 */}
      <Card title='主要功能' className={styles.featuresCard}>
        <div className={styles.featuresList}>
          {features.map((feature, index) => (
            <div key={index} className={styles.featureItem}>
              <div className={styles.featureBullet}>•</div>
              <span>{feature}</span>
            </div>
          ))}
        </div>
      </Card>

      {/* 联系方式 */}
      <List header='联系我们' className={styles.contactList}>
        <List.Item prefix={<GlobalOutline />} extra={<RightOutline />} onClick={() => handleContact('website')}>
          官方网站
        </List.Item>
        <List.Item prefix={<MailOutline />} extra={<RightOutline />} onClick={() => handleContact('email')}>
          邮箱支持
        </List.Item>
        {/* <List.Item prefix={<PhonebookOutline />} extra={<RightOutline />} onClick={() => handleContact('phone')}>
          客服热线
        </List.Item> */}
        <List.Item prefix={<FileOutline />} extra={<RightOutline />} onClick={() => Toast.show('使用文档开发中...')}>
          使用文档
        </List.Item>
      </List>

      {/* 底部信息 */}
      <div className={styles.footer}>
        <p>© 2024 DeBox Pro Tools. All rights reserved.</p>
        <p>专业的DeBox生态工具提供商</p>
      </div>

      {/* 返回按钮 */}
      <div className={styles.backButton}>
        <Button color='primary' fill='solid' block onClick={() => navigate(-1)}>
          返回
        </Button>
      </div>
    </div>
  );
};

export default About;
