import { Checkbox, Space, Avatar, DotLoading, CheckList } from 'antd-mobile';
import { FC, useCallback, useEffect, useState } from 'react';
import Empty from '@/components/empty';
import styles from './index.module.scss';
import { DeboxGroup, GetDeboxGroupsReq } from '@/typings/debox-group';
import { DeboxGroupApi } from '@/api/debox-group';

interface AccountGroupListProps {
  accountId: number;
  multiple?: boolean; // 是否多选
  defaultSelected?: boolean; // 是否默认选中第一个账号
  className?: string;
  onChange?: (accountGroupIds: number[]) => void;
}

// 获取群组列表
const fetchGroups = async (accountId: number): Promise<DeboxGroup[]> => {
  const data: GetDeboxGroupsReq = {
    page: 0,
    page_size: 0,
    all: true,
    status: true,
    account_ids: [accountId],
  };
  const response = await DeboxGroupApi.list(data);
  return response.data_list;
};

const AccountGroupList: FC<AccountGroupListProps> = ({
  accountId,
  multiple = false,
  defaultSelected = false,
  className,
  onChange,
}) => {
  const [groups, setGroups] = useState<DeboxGroup[]>([]);
  const [accountGroupIds, setAccountGroupIds] = useState<number[]>([]);
  const [loading, setLoading] = useState(false);

  // 获取群组列表
  useEffect(() => {
    const loadGroups = async () => {
      if (!accountId || accountId === 0) {
        return;
      }
      try {
        setLoading(true);
        const data = await fetchGroups(accountId);
        setGroups(data);

        // 默认第一个群组选中
        if (defaultSelected && data.length > 0) {
          setAccountGroupIds([data[0].id]);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadGroups();
  }, [accountId, defaultSelected]);

  useEffect(() => {
    if (!onChange) {
      return;
    }
    onChange(accountGroupIds);
  }, [accountGroupIds, onChange]);

  // 全选
  const handleSelectAllGroup = useCallback(
    (checked: boolean) => {
      if (checked) {
        setAccountGroupIds(groups.map((group) => group.id));
      } else {
        setAccountGroupIds([]);
      }
    },
    [groups, setAccountGroupIds],
  );

  if (loading) {
    return <DotLoading color='primary' />;
  }
  if (groups.length === 0) {
    return <Empty className={styles.empty} title='暂无数据' />;
  }

  return (
    <div className={className}>
      <div className={styles.allGroupsCheckbox}>
        {multiple ? (
          <Checkbox
            indeterminate={accountGroupIds.length > 0 && accountGroupIds.length < groups.length}
            checked={accountGroupIds.length === groups.length}
            onChange={handleSelectAllGroup}
          >
            全选
          </Checkbox>
        ) : (
          <></>
        )}
      </div>

      <CheckList
        className={styles.groupList}
        multiple={multiple}
        value={accountGroupIds}
        onChange={(val) => {
          if (multiple) {
            setAccountGroupIds(val as number[]);
          } else {
            setAccountGroupIds([val[0] as number]);
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
