import { Button } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import './index.module.less';

const Help = () => {
  const navigate = useNavigate();

  return (
    <div className='help'>
      <h1>帮助</h1>
      <Button onClick={() => navigate(-1)}>返回</Button>
    </div>
  );
};

export default Help;
