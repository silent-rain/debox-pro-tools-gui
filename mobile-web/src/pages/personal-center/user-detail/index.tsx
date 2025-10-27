import { useNavigate } from 'react-router-dom';
import { Avatar, NavBar } from 'antd-mobile';
import './index.module.less';

const UserDetail = () => {
  const navigate = useNavigate();
  const userInfo = {
    avatar: 'https://example.com/avatar.jpg',
    name: '用户名',
    bio: '欢迎来到我的个人中心！',
  };

  return (
    <div className='user-detail'>
      <NavBar onBack={() => navigate(-1)}>用户详情</NavBar>
      <div className='user-content'>
        <Avatar src={userInfo.avatar} style={{ '--size': '120px' }} />
        <h2>{userInfo.name}</h2>
        <p>{userInfo.bio}</p>
      </div>
    </div>
  );
};

export default UserDetail;
