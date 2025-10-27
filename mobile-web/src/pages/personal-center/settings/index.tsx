import { Button } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import './index.module.less';

const Settings = () => {
  const navigate = useNavigate();

  return (
    <div className='settings'>
      <h1>设置</h1>
      <Button onClick={() => navigate(-1)}>返回</Button>
    </div>
  );
};

export default Settings;
