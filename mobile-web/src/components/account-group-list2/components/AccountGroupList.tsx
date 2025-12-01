import { Checkbox, Space, Avatar, DotLoading, CheckList } from 'antd-mobile';
import { FC, useCallback, useEffect, useState, useMemo } from 'react';
import Empty from '@/components/empty';
import styles from './AccountGroupList.module.scss';
import { DeboxGroup, GetDeboxGroupsReq } from '@/typings/debox-group';
import { DeboxGroupApi } from '@/api/debox-group';

interface AccountGroupListProps {
  accounts: number[];
  selectedAccountGroups: number[];
  setSelectedAccountGroups?: (selected: number[]) => void;
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
}

// 获取群组列表
const fetchGroups = async (accountIds: number[]): Promise<DeboxGroup[]> => {
  const data: GetDeboxGroupsReq = {
    page: 0,
    page_size: 0,
    all: true,
    status: true,
    account_ids: accountIds,
  };
  const response = await DeboxGroupApi.list(data);
  return response.data_list;
};

const AccountGroupList: FC<AccountGroupListProps> = ({
  accounts,
  selectedAccountGroups,
  setSelectedAccountGroups,
  multiple = false,
  defaultSelected = false,
}) => {
  // 使用 useMemo 包装默认函数，避免每次渲染都创建新的函数引用
  const safeSetSelectedAccountGroups = useMemo(
    () => setSelectedAccountGroups || ((_selected: number[]) => {}),
    [setSelectedAccountGroups],
  );
  const [groups, setGroups] = useState<DeboxGroup[]>([]);
  const [loading, setLoading] = useState(false);

  // 获取群组列表
  useEffect(() => {
    const loadGroups = async () => {
      if (accounts.length === 0) {
        setGroups([]);
        return;
      }
      try {
        setLoading(true);
        const data = await fetchGroups(accounts);
        setGroups(data);

        // 默认第一个群组选中
        if (defaultSelected && data.length > 0) {
          safeSetSelectedAccountGroups([data[0].id]);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadGroups();
  }, [accounts, defaultSelected, safeSetSelectedAccountGroups]);

  // 全选
  const handleSelectAllGroup = useCallback(
    (checked: boolean) => {
      if (checked) {
        safeSetSelectedAccountGroups(groups.map((group) => group.id));
      } else {
        safeSetSelectedAccountGroups([]);
      }
    },
    [groups, safeSetSelectedAccountGroups],
  );

  if (loading) {
    return <DotLoading color='primary' />;
  }

  return (
    <div className='account-group'>
      <div className={styles.allGroupsCheckbox}>
        {multiple ? (
          <Checkbox
            indeterminate={selectedAccountGroups.length > 0 && selectedAccountGroups.length < groups.length}
            checked={selectedAccountGroups.length === groups.length}
            onChange={handleSelectAllGroup}
          >
            全选
          </Checkbox>
        ) : (
          <></>
        )}
      </div>

      <div className={styles.groupListDivider}></div>

      {groups.length === 0 ? <Empty title='暂无数据' /> : <></>}

      <CheckList
        className='account-list'
        defaultValue={selectedAccountGroups ? selectedAccountGroups : []}
        onChange={(val) => {
          if (multiple) {
            safeSetSelectedAccountGroups(val as number[]);
          } else {
            safeSetSelectedAccountGroups([val[0] as number]);
          }
        }}
      >
        {groups.map((item) => (
          <CheckList.Item key={item.id} value={item.id}>
            <Space align='center'>
              <Avatar src={item.pic ?? ''} />
              <span>{item.name}</span>
            </Space>
          </CheckList.Item>
        ))}
      </CheckList>
    </div>
  );
};

export default AccountGroupList;
