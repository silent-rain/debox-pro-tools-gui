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
  const [toUserIds, setToUserIds] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadAccounts = async () => {
      try {
        setToUserIds([]);
        setLoading(true);
        const data = await fetchAccountFriends(accountId);
        setAccountFriends(data);

        // 默认第一个群组选中
        if (defaultSelected && data.length > 0) {
          setToUserIds([data[0].invite_code]);
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
    onChange(toUserIds);
  }, [toUserIds, onChange]);

  // 全选
  const handleSelectAllFriend = useCallback(
    (checked: boolean) => {
      if (checked) {
        setToUserIds(accountFriends.map((friend) => friend.invite_code));
      } else {
        setToUserIds([]);
      }
    },
    [accountFriends, setToUserIds],
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
            indeterminate={toUserIds.length > 0 && toUserIds.length < accountFriends.length}
            checked={toUserIds.length === accountFriends.length}
            onChange={handleSelectAllFriend}
          >
            全选
          </Checkbox>
        ) : (
          <></>
        )}
      </div>

      <CheckList
        className={styles.friendList}
        multiple={multiple}
        value={toUserIds}
        onChange={(val) => {
          if (multiple) {
            console.log('val', val);
            setToUserIds(val as string[]);
          } else {
            setToUserIds([val[0] as string]);
          }
        }}
      >
        {accountFriends.map((item) => (
          <CheckList.Item key={item.id} value={item.invite_code}>
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
