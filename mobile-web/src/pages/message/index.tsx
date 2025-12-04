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
    <div className='send-message-management'>
      {/* 选择账号 */}
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

      <div className={styles.operationButton}>
        <Button color='primary' size='small' fill='solid' onClick={handleRefreshFriends}>
          刷新
        </Button>
      </div>

      <AccountFriendList
        accountId={accountId}
        multiple
        friendsUpdateState={friendsUpdateState}
        onChange={(toUserIds: string[]) => {
          setToUserIds(toUserIds);
        }}
      />

      <div className='send-message'>
        <Input placeholder='请输入消息内容' clearable value={content} onChange={(value) => setContent(value)} />
        <Button color='primary' size='small' fill='solid' onClick={handleSendMessage}>
          发送
        </Button>
      </div>
    </div>
  );
};

export default SendMessageManagement;
