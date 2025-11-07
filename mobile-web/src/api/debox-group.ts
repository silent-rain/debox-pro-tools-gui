// DeBox群组管理

import request from '@/utils/request';
import { SERVER } from '@/constants/http';
import {
  CreateDeboxGroupReq,
  CreateDeboxGroupResp,
  DeleteDeboxGroupReq,
  DeleteDeboxGroupResp,
  GetDeboxGroupReq,
  GetDeboxGroupResp,
  GetDeboxGroupsReq,
  GetDeboxGroupsResp,
  SyncDeboxGroupReq,
  SyncDeboxGroupResp,
  UpdateDeboxGroupReq,
  UpdateDeboxGroupResp,
  UpdateDeboxGroupStatusReq,
  UpdateDeboxGroupStatusResp,
} from '@/typings/debox-group';

export const DeboxGroupApi = {
  // 获取群组列表
  list: async (data: GetDeboxGroupsReq): Promise<GetDeboxGroupsResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-groups/list`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 获取群组信息
  info: async (data: GetDeboxGroupReq): Promise<GetDeboxGroupResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-groups/${data.id}`,
      method: 'GET',
      params: {},
    });
    return response.data;
  },

  // 创建群组
  create: async (data: CreateDeboxGroupReq): Promise<CreateDeboxGroupResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-groups`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 更新群组信息
  update: async (data: UpdateDeboxGroupReq): Promise<UpdateDeboxGroupResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-groups/update`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 更新群组状态
  updateStatus: async (data: UpdateDeboxGroupStatusReq): Promise<UpdateDeboxGroupStatusResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-groups/update-status`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 删除群组
  delete: async (data: DeleteDeboxGroupReq): Promise<DeleteDeboxGroupResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-groups/${data.id}`,
      method: 'DELETE',
      data: {},
    });
    return response.data;
  },

  // 同步DeBox群组列表
  syncGroups: async (data: SyncDeboxGroupReq): Promise<SyncDeboxGroupResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-groups/sync-groups`,
      method: 'POST',
      data,
    });
    return response.data;
  },
};
