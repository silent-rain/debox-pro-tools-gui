// DeBox账号管理

import request from '@/utils/request';
import { server } from './constant';
import {
  CreateDeboxAccountReq,
  CreateDeboxAccountResp,
  DeleteDeboxAccountReq,
  DeleteDeboxAccountResp,
  DownloadConfigReq,
  DownloadConfigResp,
  GetDeboxAccountReq,
  GetDeboxAccountResp,
  GetDeboxAccountsReq,
  GetDeboxAccountsResp,
  UpdateAccountInfoReq,
  UpdateAccountInfoResp,
  UpdateAllAccountsInfoReq,
  UpdateAllAccountsInfoResp,
  UpdateDeboxAccountReq,
  UpdateDeboxAccountResp,
  UpdateDeboxAccountStatusReq,
  UpdateDeboxAccountStatusResp,
  UploadConfigResp,
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
  updateStatus: async (data: UpdateDeboxAccountStatusReq): Promise<UpdateDeboxAccountStatusResp> => {
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

  // 更新所有账户信息
  updateAllAccountsInfo: async (data: UpdateAllAccountsInfoReq): Promise<UpdateAllAccountsInfoResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts/update-all-accounts-info`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 更新账户信息
  updateAccountInfo: async (data: UpdateAccountInfoReq): Promise<UpdateAccountInfoResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts/update-account-info`,
      method: 'PUT',
      data,
    });
    return response.data;
  },

  // 下载配置
  downloadConfig: async (data: DownloadConfigReq): Promise<DownloadConfigResp> => {
    const response = await request({
      url: `${server}/debox/debox-accounts/${data.id}/download-config`,
      method: 'GET',
      params: {},
      responseType: 'blob', // 关键：指定响应类型为 blob
    });
    return response.data;
  },

  // 上传配置文件
  uploadConfigFile: async (file: File, author: string): Promise<UploadConfigResp> => {
    const formData = new FormData();
    formData.append('file', file); // 文件字段名需与后端约定（如 'file'）
    formData.append('author', author);

    const response = await request({
      url: `${server}/debox/debox-accounts/upload-config`,
      method: 'POST',
      // headers: { // 取消headers，让 axios 自动处理 Content-Type 的设置
      //   'Content-Type': 'multipart/form-data',
      // },
      data: formData,
    });
    return response.data;
  },
};
