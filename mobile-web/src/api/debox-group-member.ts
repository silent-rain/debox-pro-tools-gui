// DeBox群组成员管理

import request from '@/utils/request';
import { SERVER } from '@/constants/http';
import {
  CreateDeboxGroupMemberReq,
  CreateDeboxGroupMemberResp,
  DeleteDeboxGroupMemberReq,
  DeleteDeboxGroupMemberResp,
  GetDeboxGroupMemberReq,
  GetDeboxGroupMemberResp,
  GetDeboxGroupMembersReq,
  GetDeboxGroupMembersResp,
  SyncDeboxGroupMemberReq,
  SyncDeboxGroupMemberResp,
  UpdateDeboxGroupMemberReq,
  UpdateDeboxGroupMemberResp,
  UpdateDeboxGroupMemberStatusReq,
  UpdateDeboxGroupMemberStatusResp,
} from '@/typings/debox-group-member';

export const DeboxGroupMemberApi = {
  // 获取群组成员列表
  list: async (data: GetDeboxGroupMembersReq): Promise<GetDeboxGroupMembersResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-group-members/list`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 获取群组成员信息
  info: async (data: GetDeboxGroupMemberReq): Promise<GetDeboxGroupMemberResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-group-members/${data.id}`,
      method: 'GET',
      params: {},
    });
    return response.data;
  },

  // 创建群组成员
  create: async (data: CreateDeboxGroupMemberReq): Promise<CreateDeboxGroupMemberResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-group-members`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 更新群组成员信息
  update: async (data: UpdateDeboxGroupMemberReq): Promise<UpdateDeboxGroupMemberResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-group-members/update`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 更新群组成员状态
  updateStatus: async (data: UpdateDeboxGroupMemberStatusReq): Promise<UpdateDeboxGroupMemberStatusResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-group-members/update-status`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 删除群组成员
  delete: async (data: DeleteDeboxGroupMemberReq): Promise<DeleteDeboxGroupMemberResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-group-members/${data.id}`,
      method: 'DELETE',
      data: {},
    });
    return response.data;
  },

  // 同步DeBox群组成员列表
  syncGroupMembers: async (data: SyncDeboxGroupMemberReq): Promise<SyncDeboxGroupMemberResp> => {
    const response = await request({
      url: `${SERVER}/debox/debox-group-members/sync-group-members`,
      method: 'POST',
      data,
    });
    return response.data;
  },
};
