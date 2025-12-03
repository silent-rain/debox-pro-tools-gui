/*用户好友管理 */

import request from '@/utils/request';
import { SERVER } from '@/constants/http';
import {
  CreateDeboxAccountFriendReq,
  CreateDeboxAccountFriendResp,
  DeleteDeboxAccountFriendReq,
  DeleteDeboxAccountFriendResp,
  GetDeboxAccountFriendReq,
  GetDeboxAccountFriendResp,
  GetDeboxAccountFriendsReq,
  GetDeboxAccountFriendsResp,
  SendPrivateMessageTextReq,
  SendPrivateMessageTextResp,
  SyncDeboxAccountFriendsReq,
  SyncDeboxAccountFriendsResp,
  UpdateDeboxAccountFriendReq,
  UpdateDeboxAccountFriendResp,
  UpdateDeboxAccountFriendStatusReq,
  UpdateDeboxAccountFriendStatusResp,
} from '@/typings/debox-account-friend';

export const DeboxAccountFriendApi = {
  // 获取账号好友列表
  list: async (data: GetDeboxAccountFriendsReq): Promise<GetDeboxAccountFriendsResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends/list`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 获取账号好友信息
  info: async (data: GetDeboxAccountFriendReq): Promise<GetDeboxAccountFriendResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends/${data.id}`,
      method: 'GET',
      params: {},
    });
    return response.data;
  },

  // 创建账号好友
  create: async (data: CreateDeboxAccountFriendReq): Promise<CreateDeboxAccountFriendResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 更新账号好友信息
  update: async (data: UpdateDeboxAccountFriendReq): Promise<UpdateDeboxAccountFriendResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends/update`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 更新账号好友状态
  updateStatus: async (data: UpdateDeboxAccountFriendStatusReq): Promise<UpdateDeboxAccountFriendStatusResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends/update-status`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 删除账号好友
  delete: async (data: DeleteDeboxAccountFriendReq): Promise<DeleteDeboxAccountFriendResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends/${data.id}`,
      method: 'DELETE',
      data: {},
    });
    return response.data;
  },

  // 同步好友列表
  syncFriends: async (data: SyncDeboxAccountFriendsReq): Promise<SyncDeboxAccountFriendsResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends/sync-friends`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 发送私聊文本消息
  sendPrivateMessageText: async (data: SendPrivateMessageTextReq): Promise<SendPrivateMessageTextResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-friends/send-private-message-text`,
      method: 'POST',
      data,
    });
    return response.data;
  },
};
