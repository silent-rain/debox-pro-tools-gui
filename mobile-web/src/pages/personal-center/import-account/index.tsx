import { Button } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import './index.module.less';

const ImportAccount = () => {
  const navigate = useNavigate();

  return (
    <div className='import-account'>
      <h1>导入账号</h1>
      <Button onClick={() => navigate(-1)}>返回</Button>
    </div>
  );
};

export default ImportAccount;
