import { FC, useEffect, useState } from 'react';
import { Avatar, DotLoading, List, Switch } from 'antd-mobile';
import { CheckOutline, CloseOutline } from 'antd-mobile-icons';
import styles from './AccountFriendList.module.scss';
import Empty from '@/components/empty';
import { DeboxAccountFriendApi } from '@/api';
import { DeboxAccountFriend, GetDeboxAccountFriendsReq } from '@/typings/debox-account-friend';

interface AccountFriendListProps {
  accountId: number;
  friendsUpdateState: number;
}

// 获取账号好友列表
const fetchAccountFriends = async (accountId: number): Promise<DeboxAccountFriend[]> => {
  if (!accountId || accountId === 0) {
    return [];
  }
  const data: GetDeboxAccountFriendsReq = {
    page: 0,
    page_size: 0,
    all: true,
    status: true,
    account_ids: [accountId],
  };
  const response = await DeboxAccountFriendApi.list(data);
  return response.data_list;
};

// 更新账号好友状态
const updateAccountFriendStatus = async (friendId: number, status: boolean) => {
  const data = await DeboxAccountFriendApi.updateStatus({
    id: friendId,
    status,
  });
  return data;
};

const AccountFriendList: FC<AccountFriendListProps> = ({ accountId, friendsUpdateState }) => {
  const [accountFriends, setAccountFriends] = useState<DeboxAccountFriend[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccountFriends(accountId);
        setAccountFriends(data);
      } catch (err) {
        console.error(`fetchFriends error: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [accountId, friendsUpdateState]);

  const handleSwitchChange = async (friendId: number, checked: boolean) => {
    await updateAccountFriendStatus(friendId, checked);

    const data = await fetchAccountFriends(accountId);
    setAccountFriends(data);
  };

  if (loading) {
    return <DotLoading color='primary' />;
  }

  if (accountId === 0) {
    return <Empty title='请先选择账号' description='暂无数据' />;
  }

  if (accountFriends.length === 0) {
    return <Empty />;
  }

  return (
    <>
      <List className={styles.friendList}>
        {accountFriends.map((accountFriend) => (
          <List.Item
            key={accountFriend.id}
            prefix={<Avatar className={styles.friendAvatar} src={accountFriend.avatar ?? ''} />}
            extra={
              <Switch
                checkedText={<CheckOutline fontSize={18} />}
                uncheckedText={<CloseOutline fontSize={18} />}
                defaultChecked={accountFriend.status}
                onChange={(checked) => handleSwitchChange(accountFriend.id, checked)}
              />
            }
          >
            {accountFriend.name}
          </List.Item>
        ))}
      </List>
    </>
  );
};

export default AccountFriendList;