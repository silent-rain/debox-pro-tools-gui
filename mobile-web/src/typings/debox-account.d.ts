// Debox Account

// DeBox账号表
export interface DeboxAccount {
  id: number; // 账号ID
  user_id: number; // 用户ID
  name: string; // 账号名称
  avatar: string; // 账号头像
  api_key: string; // 开发者 API Key，在DeBox开放平台获取
  app_secret: string; // 开发者 App Secret，在DeBox开放平台获取
  access_token: string; // 登录授权, 有效期较短
  web_token: string; // WEB登录授权
  debox_user_id: string; //  DeBox 用户ID
  wallet_address: string; // 用户钱包地址
  api_key_status: boolean; // ApiKey 状态(false:无效,true:有效)
  access_token_status: boolean; // Access Token 状态(false:无效,true:有效)
  web_token_status: boolean; // Web Token 状态(false:无效,true:有效)
  desc: string; // 描述信息
  status: boolean; // 状态(false:停用,true:正常)
  created_at: string; // 创建时间
  updated_at: string; // 更新时间
}

// 查询DeBox账号列表 请求体
export interface GetDeboxAccountsReq {
  page?: number; // 当前分页
  page_size?: number; // 页面大小
  start_time?: string; // 开始时间
  end_time?: string; // 结束时间
  user_id: number; // 用户ID
  all: boolean; // 返回所有数据
}

// 查询DeBox账号列表 响应体
export interface GetDeboxAccountsResp {
  data_list: DeboxAccount[];
  total: number;
}

// 查询DeBox账号信息 请求体
export interface GetDeboxAccountReq {
  id: number; // DeBox账号ID
}

// 查询DeBox账号信息 响应体
export type GetDeboxAccountResp = DeboxAccount;

// 添加DeBox账号 请求体
export interface CreateDeboxAccountReq {
  user_id: number; // 用户ID
  app_id: string; // 应用唯一标识，在DeBox开放平台申请
  api_key: string; // 开发者 API Key，在DeBox开放平台获取
  app_secret: string; // 开发者 App Secret，在DeBox开放平台获取
  access_token: string; // 登录授权, 有效期较短
  web_token: string; // WEB登录授权
  debox_user_id: string; // DeBox 用户ID
  wallet_address: string; // 用户钱包地址
  api_key_status: boolean; // ApiKey 状态(false:无效,true:有效)
  access_token_status: boolean; // Access Token 状态(false:无效,true:有效)
  web_token_status: boolean; // Web Token 状态(false:无效,true:有效)
  desc: string; // 描述信息
  status: boolean; // 状态(false:停用,true:正常)
}

// 添加DeBox账号 响应体
// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface CreateDeboxAccountResp {}

// 更新DeBox账号 请求体
export type UpdateDeboxAccountReq = DeboxAccount;

// 更新DeBox账号 响应体
// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface UpdateDeboxAccountResp {}

// 更新DeBox账号状态 请求体
export interface UpdateDeboxAccountStatusReq {
  id: number; // DeBox账号ID
  status: boolean; // 状态(false:停用,true:正常)
}

// 更新DeBox账号状态 响应体
// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface UpdateDeboxAccountStatusResp {}

// 删除数据 请求体
export interface DeleteDeboxAccountReq {
  id: number; // DeBox账号ID
}

// 删除数据 响应体
// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface DeleteDeboxAccountResp {}
