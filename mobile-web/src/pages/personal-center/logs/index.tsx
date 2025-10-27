import { Button } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import './index.module.less';

const Logs = () => {
  const navigate = useNavigate();

  return (
    <div className='logs'>
      <h1>日志</h1>
      <Button onClick={() => navigate(-1)}>返回</Button>
    </div>
  );
};

export default Logs;
