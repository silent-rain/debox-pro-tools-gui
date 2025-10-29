// DeBox账号管理

import request from '@/utils/request';
import { server } from './constant';
import {
  CreateDeboxAccountReq,
  CreateDeboxAccountResp,
  DeleteDeboxAccountReq,
  DeleteDeboxAccountResp,
  GetDeboxAccountReq,
  GetDeboxAccountResp,
  GetDeboxAccountsReq,
  GetDeboxAccountsResp,
  UpdateDeboxAccountReq,
  UpdateDeboxAccountResp,
  UpdateDeboxAccountStatusReq,
  UpdateDeboxAccountStatusResp,
} from '@/typings/debox-account';

export const DeboxAccountApi = {
  // 获取账号列表
  list: async (data: GetDeboxAccountsReq): Promise<GetDeboxAccountsResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts`,
      method: 'GET',
      params: data,
    });
    return response.data;
  },

  // 获取账号信息
  info: async (data: GetDeboxAccountReq): Promise<GetDeboxAccountResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts/${data.id}`,
      method: 'GET',
      params: {},
    });
    return response.data;
  },

  // 创建账号
  create: async (data: CreateDeboxAccountReq): Promise<CreateDeboxAccountResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts`,
      method: 'POST',
      data,
    });
    return response.data;
  },

  // 更新账号信息
  update: async (data: UpdateDeboxAccountReq): Promise<UpdateDeboxAccountResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts/${data.id}`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 更新账号信息
  update_status: async (data: UpdateDeboxAccountStatusReq): Promise<UpdateDeboxAccountStatusResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts/${data.id}/status`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 删除账号
  delete: async (data: DeleteDeboxAccountReq): Promise<DeleteDeboxAccountResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts/${data.id}`,
      method: 'DELETE',
      data: {},
    });
    return response.data;
  },
};
