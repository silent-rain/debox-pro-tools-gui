import { CheckList, SearchBar, InfiniteScroll } from 'antd-mobile';
import { FC, useState } from 'react';
import styles from './UserSearchList.module.scss';
import { DeboxUserSearchReq, UserSearch } from '@/typings/debox-account-follow';
import { DeboxAccountFollowApi } from '@/api';
import Empty from '@/components/empty';

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
  return await DeboxAccountFollowApi.deboxUserSearch(data);
};

const UserSearchList: FC<UserSearchListProps> = ({ accountId, userIds, setUserIds }) => {
  const [users, setUsers] = useState<UserSearch[]>([]);
  const [searchPage, setSearchPage] = useState(1);
  const [searchTerm, setSearchTerm] = useState('');
  const [hasMore, setHasMore] = useState(false);

  const loadMore = async () => {
    if (!searchTerm) {
      setUsers([]);
      return;
    }

    const data = await deboxUserSearch(accountId, searchTerm, searchPage);
    const dataList = data.data_list;
    setUsers((val) => [...val, ...dataList]);
    setHasMore(dataList.length > 0);
    setSearchPage((val) => val + 1);
  };

  return (
    <div className='user-serch-list'>
      <SearchBar
        placeholder='请输入检索用户'
        onSearch={(val) => {
          setSearchTerm(val.trim());
          loadMore();
        }}
      />

      {users.length === 0 ? <Empty className={styles.empty} title='' /> : <></>}

      <CheckList
        className={styles.allAccounts}
        multiple
        defaultValue={userIds ? userIds : []}
        onChange={(val) => {
          setUserIds(val as number[]);
        }}
      >
        {users.map((item) => (
          <CheckList.Item key={item.user_id} value={item.user_id}>
            {item.name}
          </CheckList.Item>
        ))}
      </CheckList>

      <InfiniteScroll loadMore={loadMore} hasMore={hasMore} />
    </div>
  );
};

export default UserSearchList;
