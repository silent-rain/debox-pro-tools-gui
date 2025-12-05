import { useState } from 'react';
import { Button, Dropdown, Toast } from 'antd-mobile';
import AccountList from '@/components/account-list';
import AccountFriendList from './components/AccountFriendList';
import AddGroupMember from './components/AddGroupMember';
import styles from './index.module.scss';
import CreateGroup from './components/CreateGroup';

const CreateGroupManagement = () => {
  const [accountId, setAccountId] = useState<number>(0);
  const [deboxUserIds, setDeboxUserIds] = useState<string[]>([]);
  const [friendsUpdateState, setFriendsUpdateState] = useState<number>(0);
  const [showGroupSelectModal, setShowGroupSelectModal] = useState(false);
  const [showCreateGroupModal, setShowCreateGroupModal] = useState(false);

  // 刷新好友列表
  const handleRefreshFriends = () => {
    setFriendsUpdateState((prev) => prev + 1);
  };

  // 打开选择群组弹窗
  const handleOpenGroupSelect = async () => {
    if (!accountId || accountId === 0) {
      Toast.show('请先选择账号');
      return;
    }
    if (deboxUserIds.length === 0) {
      Toast.show('请先选择好友');
      return;
    }
    setShowGroupSelectModal(true);
  };

  // 打开新建群组弹窗
  const handleOpenCreateGroup = () => {
    if (!accountId || accountId === 0) {
      Toast.show('请先选择账号');
      return;
    }
    if (deboxUserIds.length === 0) {
      Toast.show('请先选择好友');
      return;
    }
    setShowCreateGroupModal(true);
  };

  return (
    <div className='create-group-management'>
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
        <Button color='success' size='small' fill='solid' onClick={handleOpenGroupSelect}>
          添加成员
        </Button>
        <Button color='warning' size='small' fill='solid' onClick={handleOpenCreateGroup}>
          新建群组
        </Button>
      </div>

      {/* 好友列表 */}
      <AccountFriendList
        accountId={accountId}
        multiple
        friendsUpdateState={friendsUpdateState}
        onChange={(deboxUserIds: string[]) => {
          setDeboxUserIds(deboxUserIds);
        }}
      />

      {/* 选择群组弹窗 */}
      <AddGroupMember
        visible={showGroupSelectModal}
        accountId={accountId}
        deboxUserIds={deboxUserIds}
        onClose={() => setShowGroupSelectModal(false)}
        onSubmit={() => {
          handleRefreshFriends();
          setDeboxUserIds([]);
        }}
      />

      {/* 新建群组 */}
      <CreateGroup
        visible={showCreateGroupModal}
        accountId={accountId}
        deboxUserIds={deboxUserIds}
        onClose={() => setShowCreateGroupModal(false)}
        onSubmit={() => {
          handleRefreshFriends();
          setDeboxUserIds([]);
        }}
      />
    </div>
  );
};

export default CreateGroupManagement;
