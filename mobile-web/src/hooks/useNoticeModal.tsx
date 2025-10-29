import { Modal, ModalShowProps } from 'antd-mobile';
import { useCallback } from 'react';

type NoticeType = 'default' | 'success' | 'alert' | 'error' | 'info';

// 定义颜色配置
const noticeColors = {
  default: {
    backgroundColor: '#f6f6f6', // 默认背景色
    textColor: '#000000', // 默认字体颜色
    icon: '🔔',
  },
  success: {
    backgroundColor: '#f6ffed', // 成功背景色
    textColor: '#52c41a', // 成功字体颜色
    icon: '✅',
  },
  alert: {
    backgroundColor: '#fffbe6', // 警告背景色
    textColor: '#faad14', // 警告字体颜色
    icon: '⚠️',
  },
  error: {
    backgroundColor: '#fff2f0', // 错误背景色
    textColor: '#ff4d4f', // 错误字体颜色
    icon: '❌',
  },
  info: {
    backgroundColor: '#e6f7ff', // 信息背景色
    textColor: '#1890ff', // 信息字体颜色
    icon: 'ℹ️',
  },
};

/*
const { showNotice } = useNoticeModal();

<button onClick={() => showNotice('默认消息', 'default')}>默认</button>
<button onClick={() => showNotice('成功消息', 'success')}>成功</button>
<button onClick={() => showNotice('警告消息', 'alert')}>警告</button>
<button onClick={() => showNotice('错误消息', 'error')}>错误</button>
<button onClick={() => showNotice('信息消息', 'info')}>信息</button>
*/

export const useNoticeModal = () => {
  const showNotice = useCallback((content: string, type: NoticeType = 'default') => {
    const { backgroundColor, textColor, icon } = noticeColors[type];

    const modalProps: ModalShowProps = {
      content: (
        <div
          style={{
            backgroundColor,
            color: textColor,
            padding: '12px 16px',
            borderRadius: '4px',
            display: 'flex',
            alignItems: 'center',
          }}
        >
          <span>{icon}</span>
          <span style={{ marginLeft: '8px' }}>{content}</span>
        </div>
      ),
      closeOnMaskClick: true,
      // 设置 Modal 固定在顶部
      bodyStyle: {
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        margin: 0,
        padding: 0,
        width: '100%',
        backgroundColor, // 背景色填充
        color: textColor,
      },
      // 遮罩层样式（可选）
      maskStyle: {
        // backgroundColor: 'rgba(0, 0, 0, 0.5)',
      },
    };

    // 调用 Modal 显示消息
    Modal.show(modalProps);
  }, []);

  return { showNotice };
};
