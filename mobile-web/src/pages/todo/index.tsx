import { useState } from 'react';
import reactLogo from '@/assets/react.svg';
import viteLogo from '/vite.svg';
import './index.module.less';
import { hello } from '@/api/comfyui';
import { Button, Input } from 'antd-mobile';

export default function Home() {
  const [name, setName] = useState('');
  const [hi, setHi] = useState('');

  const handleHello = async () => {
    const resp = await hello({ name: name });
    console.error('resp: ', resp.data);
    setHi(resp.data);
  };

  return (
    <>
      <div>
        <a href='https://vite.dev' target='_blank' rel='noreferrer'>
          <img src={viteLogo} className='logo' alt='Vite logo' />
        </a>
        <a href='https://react.dev' target='_blank' rel='noreferrer'>
          <img src={reactLogo} className='logo react' alt='React logo' />
        </a>
      </div>
      <h1>Vite + React - TODO</h1>
      <div className='card'>
        <label> 请输入名称: </label>
        <Input
          placeholder='请输入名称'
          value={name}
          onChange={(val) => {
            setName(val);
          }}
        />

        <Button color='primary' fill='solid' onClick={handleHello}>
          Send
        </Button>

        <p>{hi}</p>
      </div>
    </>
  );
}
