import { Button } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import './index.module.less';

const About = () => {
  const navigate = useNavigate();

  return (
    <div className='about'>
      <h1>关于</h1>
      <Button onClick={() => navigate(-1)}>返回</Button>
    </div>
  );
};

export default About;
