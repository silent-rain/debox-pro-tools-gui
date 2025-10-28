import { createRoot } from 'react-dom/client';
import App from './App.tsx';

import VConsole from 'vconsole';

// 手机端调试
const vConsole = new VConsole();
console.log(vConsole);

// 重置浏览器样式
import 'normalize.css';

// 自定义样式
import './main.css';

createRoot(document.getElementById('root')!).render(<App />);
