import { useState } from 'react';
import { Button, Dropdown, Input } from 'antd-mobile';
import AccountList from '@/components/account-list';
import AccountFriendList from './components/AccountFriendList';
import styles from './index.module.scss';
import { DeboxAccountFriendApi } from '@/api';
import { SendPrivateMessageTextReq } from '@/typings/debox-account-friend';

// 发送私信
const handleSendPrivateMessageText = async (accountId: number, toUserIds: string[], content: string) => {
  const data: SendPrivateMessageTextReq = {
    account_id: accountId,
    to_user_ids: toUserIds,
    content: content,
  };
  await DeboxAccountFriendApi.sendPrivateMessageText(data);
};

const SendMessageManagement = () => {
  const [accountId, setAccountId] = useState<number>(0);
  const [toUserIds, setToUserIds] = useState<string[]>([]);
  const [friendsUpdateState, setFriendsUpdateState] = useState<number>(0);
  const [content, setContent] = useState<string>('');

  // 刷新好友列表
  const handleRefreshFriends = () => {
    setFriendsUpdateState((prev) => prev + 1);
  };

  // 发送消息
  const handleSendMessage = async () => {
    await handleSendPrivateMessageText(accountId, toUserIds, content);
    setContent(''); // 清空输入框
  };

  return (
    <div className={styles.sendMessageManagement}>
      {/* 顶部操作栏：账号选择和刷新按钮 */}
      <div className={styles.topBar}>
        <div className={styles.accountSelector}>
          <Dropdown defaultActiveKey='account'>
            <Dropdown.Item key='account' title='选择账号'>
              <AccountList
                defaultSelected
                onChange={(accountIds) => {
                  setAccountId(accountIds[0]);
                }}
              />
            </Dropdown.Item>
          </Dropdown>
        </div>
        <Button
          color='primary'
          size='small'
          fill='solid'
          onClick={handleRefreshFriends}
          className={styles.refreshButton}
        >
          刷新
        </Button>
      </div>

      {/* 消息输入区域 - 移到顶部下方 */}
      <div className={styles.messageSection}>
        <div className={styles.inputContainer}>
          <Input
            placeholder='请输入消息内容'
            clearable
            value={content}
            onChange={(value) => setContent(value)}
            className={styles.messageInput}
          />
          <Button
            color='primary'
            size='small'
            fill='solid'
            onClick={handleSendMessage}
            disabled={!content.trim() || toUserIds.length === 0}
            className={styles.sendButton}
          >
            发送
          </Button>
        </div>
        {toUserIds.length > 0 && <div className={styles.selectedInfo}>已选择 {toUserIds.length} 个好友</div>}
      </div>

      {/* 好友列表区域 */}
      <div className={styles.friendsSection}>
        <AccountFriendList
          accountId={accountId}
          multiple
          friendsUpdateState={friendsUpdateState}
          onChange={(toUserIds: string[]) => {
            setToUserIds(toUserIds);
          }}
        />
      </div>
    </div>
  );
};

export default SendMessageManagement;
