import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.tsx';

import VConsole from 'vconsole';

const vConsole = new VConsole();
console.log(vConsole);

// 重置浏览器样式
import 'normalize.css';

// 自定义样式
import './main.less';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
