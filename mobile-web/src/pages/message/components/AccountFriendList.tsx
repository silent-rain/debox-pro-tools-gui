import { FC, useCallback, useEffect, useState } from 'react';
import { Avatar, Checkbox, CheckList, DotLoading, Space } from 'antd-mobile';
import styles from './AccountFriendList.module.scss';
import Empty from '@/components/empty';
import { DeboxAccountFriendApi } from '@/api';
import { DeboxAccountFriend, GetDeboxAccountFriendsReq } from '@/typings/debox-account-friend';

interface AccountFriendListProps {
  accountId: number;
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
  friendsUpdateState: number;
  onChange?: (accountFriendIds: string[]) => void;
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

const AccountFriendList: FC<AccountFriendListProps> = ({
  accountId,
  multiple,
  defaultSelected,
  friendsUpdateState,
  onChange,
}) => {
  const [accountFriends, setAccountFriends] = useState<DeboxAccountFriend[]>([]);
  const [accountFriendIds, setAccountFriendIds] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setLoading(true);
        const data = await fetchAccountFriends(accountId);
        setAccountFriends(data);

        // 默认第一个群组选中
        if (defaultSelected && data.length > 0) {
          setAccountFriendIds([data[0].debox_user_id]);
        }
      } catch (err) {
        console.error(`fetchFriends error: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [accountId, defaultSelected, friendsUpdateState]);

  useEffect(() => {
    if (!onChange) {
      return;
    }
    onChange(accountFriendIds);
  }, [accountFriendIds, onChange]);

  // 全选
  const handleSelectAllFriend = useCallback(
    (checked: boolean) => {
      if (checked) {
        setAccountFriendIds(accountFriends.map((friend) => friend.debox_user_id));
      } else {
        setAccountFriendIds([]);
      }
    },
    [accountFriends, setAccountFriendIds],
  );

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
    <div className='account-friend-list'>
      <div className={styles.allFriendsCheckbox}>
        {multiple ? (
          <Checkbox
            indeterminate={accountFriendIds.length > 0 && accountFriendIds.length < accountFriends.length}
            checked={accountFriendIds.length === accountFriends.length}
            onChange={handleSelectAllFriend}
          >
            全选
          </Checkbox>
        ) : (
          <></>
        )}
      </div>

      <CheckList
        className='group-list'
        value={accountFriendIds}
        onChange={(val) => {
          if (multiple) {
            setAccountFriendIds(val as string[]);
          } else {
            setAccountFriendIds([val[0] as string]);
          }
        }}
      >
        {accountFriends.map((item) => (
          <CheckList.Item key={item.debox_user_id} value={item.debox_user_id}>
            <Space align='center'>
              <Avatar src={item.avatar ?? ''} />
              <span>{item.name}</span>
            </Space>
          </CheckList.Item>
        ))}
      </CheckList>
    </div>
  );
};

export default AccountFriendList;
