/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html', // 如果有使用到 HTML 文件
    './src/**/*.{html,js,ts,jsx,tsx,css}', // 扫描 src 目录下所有 js,ts,jsx,tsx 文件
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
