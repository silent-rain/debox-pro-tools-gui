import { DotLoading, CheckList, SearchBar } from 'antd-mobile';
import { FC, useEffect, useState } from 'react';
import styles from './UserSearchList.module.scss';
import { DeboxUserSearchReq, UserSearch } from '@/typings/debox-account-follows';
import { DeboxAccountFollowFollowApi } from '@/api';

interface UserSearchListProps {
  accountId: number;
  userIds: number[];
  setUserIds: (selected: number[]) => void;
}

// 用户搜索
const deboxUserSearch = async (account_id: number, search: string, page: number) => {
  let data: DeboxUserSearchReq = {
    account_id: account_id,
    search: search,
    page: page,
    size: 50,
  };
  return await DeboxAccountFollowFollowApi.deboxUserSearch(data);
};

const UserSearchList: FC<UserSearchListProps> = ({ accountId, userIds, setUserIds }) => {
  const [users, setUsers] = useState<UserSearch[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchPage, _setSearchPage] = useState(1);
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    const loadAccounts = async () => {
      if (!searchTerm) {
        setUsers([]);
        return;
      }
      try {
        setLoading(true);
        const data = await deboxUserSearch(accountId, searchTerm, searchPage);
        setUsers(data.data_list);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadAccounts();
  }, [setUsers, accountId, searchTerm, searchPage]);

  if (loading) {
    return <DotLoading color='primary' />;
  }

  return (
    <div className='user-serch-list'>
      <SearchBar
        placeholder='请输入检索用户'
        onSearch={(val) => {
          setSearchTerm(val.trim());
        }}
      />

      <CheckList
        className={styles.allAccounts}
        multiple
        defaultValue={userIds ? userIds : []}
        onChange={(val) => {
          console.log('selected user ids: ', val);
          setUserIds(val as number[]);
        }}
      >
        {users.map((item) => (
          <CheckList.Item key={item.user_id} value={item.user_id}>
            {item.name}
          </CheckList.Item>
        ))}
      </CheckList>
    </div>
  );
};

export default UserSearchList;
