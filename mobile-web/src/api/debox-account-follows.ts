/*用户关注管理 */

import request from '@/utils/request';
import { SERVER } from '@/constants/http';
import {
  CreateDeboxAccountFollowReq,
  CreateDeboxAccountFollowResp,
  DeleteDeboxAccountFollowReq,
  DeleteDeboxAccountFollowResp,
  GetDeboxAccountFollowReq,
  GetDeboxAccountFollowResp,
  GetDeboxAccountFollowsReq,
  GetDeboxAccountFollowsResp,
  SyncDeboxAccountFollowsReq,
  SyncDeboxAccountFollowsResp,
  UpdateDeboxAccountFollowReq,
  UpdateDeboxAccountFollowResp,
  UpdateDeboxAccountFollowStatusReq,
  UpdateDeboxAccountFollowStatusResp,
} from '@/typings/debox-account-follows';

export const DeboxAccountFollowFollowApi = {
  // 获取账号关注人列表
  list: async (data: GetDeboxAccountFollowsReq): Promise<GetDeboxAccountFollowsResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-follows/list`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 获取账号关注人信息
  info: async (data: GetDeboxAccountFollowReq): Promise<GetDeboxAccountFollowResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-follows/${data.id}`,
      method: 'GET',
      params: {},
    });
    return response.data;
  },

  // 创建账号关注人
  create: async (data: CreateDeboxAccountFollowReq): Promise<CreateDeboxAccountFollowResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-follows`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 更新账号关注人信息
  update: async (data: UpdateDeboxAccountFollowReq): Promise<UpdateDeboxAccountFollowResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-follows/update`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 更新账号关注人状态
  updateStatus: async (data: UpdateDeboxAccountFollowStatusReq): Promise<UpdateDeboxAccountFollowStatusResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-follows/update-status`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 删除账号关注人
  delete: async (data: DeleteDeboxAccountFollowReq): Promise<DeleteDeboxAccountFollowResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-follows/${data.id}`,
      method: 'DELETE',
      data: {},
    });
    return response.data;
  },

  // 同步关注人列表
  syncFollows: async (data: SyncDeboxAccountFollowsReq): Promise<SyncDeboxAccountFollowsResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-account-follows/sync-follows`,
      method: 'POST',
      data,
    });
    return response.data;
  },
};
