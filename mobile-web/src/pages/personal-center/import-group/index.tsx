import { Button } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import './index.module.less';

const ImportGroup = () => {
  const navigate = useNavigate();

  return (
    <div className='import-group'>
      <h1>导入群组</h1>
      <Button onClick={() => navigate(-1)}>返回</Button>
    </div>
  );
};

export default ImportGroup;
