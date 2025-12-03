import { useState } from 'react';
import { Button, Dropdown } from 'antd-mobile';
import AccountList from '@/components/account-list';
import AccountFriendList from './components/AccountFriendList';
import styles from './index.module.scss';
import { DeboxAccountFriendApi } from '@/api';

// 同步好友列表
const syncFriends = async (accountId: number) => {
  const data = {
    account_ids: [accountId],
  };
  await DeboxAccountFriendApi.syncFriends(data);
};

const FriendManagement = () => {
  const [accountId, setAccountId] = useState<number>(0);
  const [friendsUpdateState, setFriendsUpdateState] = useState<number>(0);

  // 同步好友
  const handleSyncFriends = async () => {
    if (accountId === 0) {
      return;
    }

    await syncFriends(accountId);

    setFriendsUpdateState((prev) => prev + 1);
  };

  // 刷新好友列表
  const handleRefreshFriends = () => {
    setFriendsUpdateState((prev) => prev + 1);
  };

  return (
    <div className='account-friend-management'>
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

        <Button color='primary' size='small' fill='solid' onClick={handleSyncFriends}>
          同步好友
        </Button>
      </div>

      <AccountFriendList accountId={accountId} friendsUpdateState={friendsUpdateState} />
    </div>
  );
};

export default FriendManagement;
