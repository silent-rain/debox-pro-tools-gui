import { useState } from 'react';
import { Modal, Toast } from 'antd-mobile';
import AccountGroupList from '@/components/account-group-list';
import styles from './AddGroupMember.module.scss';
import { DeboxGroupMemberApi } from '@/api';
import { AddDeboxGroupMemberReq } from '@/typings/debox-group-member';

interface AddGroupMemberProps {
  visible: boolean;
  accountId: number;
  deboxUserIds: string[];
  onClose: () => void;
  onSubmit: () => void;
}

const AddGroupMember = ({ visible, accountId, deboxUserIds, onClose, onSubmit }: AddGroupMemberProps) => {
  const [groupId, setGroupId] = useState<number>(0);

  // 提交添加成员到群组
  const handleAddMembersToGroups = async () => {
    if (groupId === 0) {
      Toast.show('请选择群组');
      return;
    }

    try {
      // 这里需要调用添加成员到群组的API
      const data: AddDeboxGroupMemberReq = {
        account_id: accountId,
        group_id: groupId,
        debox_user_ids: deboxUserIds,
      };
      await DeboxGroupMemberApi.addDeboxGroupMember(data);
      setGroupId(0);
      onSubmit();
      onClose();
    } catch (err) {
      console.error('添加成员失败:', err);
      Toast.show('添加成员失败');
    }
  };

  return (
    <Modal
      title='选择群组'
      visible={visible}
      showCloseButton
      onClose={() => {
        onClose();
      }}
      content={
        <div className={styles.contentContainer}>
          {/* 群组列表 */}
          <AccountGroupList
            accountId={accountId}
            onChange={(accountGroupIds) => {
              setGroupId(accountGroupIds[0]);
            }}
          />
        </div>
      }
      actions={[
        {
          key: 'confirm',
          text: '提交',
          primary: true,
          onClick: handleAddMembersToGroups,
        },
      ]}
    />
  );
};

export default AddGroupMember;
