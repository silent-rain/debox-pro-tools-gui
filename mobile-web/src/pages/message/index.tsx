import { useState } from 'react';
import { Button, Dropdown, Input, TextArea } from 'antd-mobile';
import AccountList from '@/components/account-list';
import AccountFriendList from './components/AccountFriendList';
import styles from './index.module.scss';

const SendMessageManagement = () => {
  const [accountId, setAccountId] = useState<number>(0);
  const [accountFriendDeboxUserIds, setAccountFriendDeboxUserIds] = useState<string[]>([]);
  const [friendsUpdateState, setFriendsUpdateState] = useState<number>(0);

  // 刷新好友列表
  const handleRefreshFriends = () => {
    setFriendsUpdateState((prev) => prev + 1);
  };

  // 发送消息
  const handleSendMessage = () => {
    console.log('accountFriendDeboxUserIds', accountFriendDeboxUserIds);
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
        onChange={(accountFriendIds: string[]) => {
          setAccountFriendDeboxUserIds(accountFriendIds);
        }}
      />

      <div className='send-message'>
        <Input placeholder='请输入消息内容' clearable />
        <Button color='primary' size='small' fill='solid' onClick={handleSendMessage}>
          发送
        </Button>
      </div>
    </div>
  );
};

export default SendMessageManagement;
