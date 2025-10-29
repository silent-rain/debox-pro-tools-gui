import { Toast, ToastShowProps } from 'antd-mobile';
import { useCallback } from 'react';

type NoticeType = 'default' | 'success' | 'alert' | 'error' | 'info';

// 定义颜色配置
const noticeColors = {
  default: {
    backgroundColor: '#f6f6f6',
    textColor: '#000000',
    icon: '🔔',
  },
  success: {
    backgroundColor: '#f6ffed',
    textColor: '#52c41a',
    icon: '✅',
  },
  alert: {
    backgroundColor: '#fffbe6',
    textColor: '#faad14',
    icon: '⚠️',
  },
  error: {
    backgroundColor: '#fff2f0',
    textColor: '#ff4d4f',
    icon: '❌',
  },
  info: {
    backgroundColor: '#e6f7ff',
    textColor: '#1890ff',
    icon: 'ℹ️',
  },
};

/*
const { showNotice } = useNoticeToast();

<button onClick={() => showNotice('默认消息', 'default')}>默认</button>
<button onClick={() => showNotice('成功消息', 'success')}>成功</button>
<button onClick={() => showNotice('警告消息', 'alert')}>警告</button>
<button onClick={() => showNotice('错误消息', 'error')}>错误</button>
<button onClick={() => showNotice('信息消息', 'info')}>信息</button>
*/

export const useNoticeToast = () => {
  const showNotice = useCallback((content: string, type: NoticeType = 'default') => {
    const { backgroundColor, textColor, icon } = noticeColors[type];

    const toastShowProps: ToastShowProps = {
      content: (
        <div
          style={{
            backgroundColor,
            color: textColor,
            padding: '12px 16px',
            borderRadius: '4px',
            display: 'flex',
            alignItems: 'center',
            border: 'none', // 消除边框
            boxShadow: 'none', // 消除阴影
          }}
        >
          <span>{icon}</span>
          <span style={{ marginLeft: '8px' }}>{content}</span>
        </div>
      ),
      position: 'top', // 固定在顶部显示
      maskClickable: true, // 点击遮罩层可关闭
      maskStyle: {
        backgroundColor: 'rgba(0, 0, 0, 0)', // 透明遮罩层
        margin: 0,
        padding: 0,
      },
      //   duration: 0,
    };

    // 使用 Toast 显示消息
    Toast.show(toastShowProps);
    Toast.config({
      duration: 0,
    });
  }, []);

  return { showNotice };
};
