/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html', // 如果有使用到 HTML 文件
    './src/**/*.{html,js,ts,jsx,tsx,css}', // 扫描 src 目录下所有 js,ts,jsx,tsx 文件
  ],
  prefix: 'tw-', // 为所有 Tailwind 类添加前缀，避免与其他框架的类冲突
  preflight: false, // 禁用全局样式重置
  theme: {
    extend: {},
  },
  plugins: [],
};
