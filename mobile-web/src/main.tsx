import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.tsx';

// 仅在开发环境中引入 vconsole
if (import.meta.env.MODE === 'development') {
  import('vconsole').then(({ default: VConsole }) => {
    new VConsole();
  });
}

// 重置浏览器样式
import 'normalize.css';

// 自定义样式
import './main.less';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
