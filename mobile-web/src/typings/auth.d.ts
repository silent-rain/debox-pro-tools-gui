import { UserType, Gender } from '@/enums/auth';

// 登陆 请求体
export interface LoginReq {
  user_type: UserType; // 用户类型
  username?: string; // 用户名
  phone?: string; // 手机号码
  email?: string; // 邮箱
  blockchain_wallet?: string; // 区块链钱包
  password: string; // 密码
  captcha_id: string; // 验证码ID
  captcha: string; // 验证码
}

// 登陆 响应体
export interface LoginResp {
  user_id: number; // 用户id
  username: string; // 用户名
  token: string; // token
  avatar: string; // 头像地址
}

// 注册用户 请求体
export interface RegisterReq {
  register_type: UserType; // 注册用户类型
  phone?: string; // 手机号码
  email?: string; // 邮箱
  blockchain_wallet?: string; // 区块链钱包
  password: string; // 密码
  password2: string; // 密码

  username: string; // 用户名称
  real_name?: string; // 真实姓名
  gender: number; // 性别(0:保密,1:女,2:男)
  age?: Gender; // 年龄
  date_birth?: string; // 出生日期
  avatar?: string; // 头像URL

  captcha_id: string; // 验证码ID
  captcha: string; // 验证码
}

// 注册用户 响应体
// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface RegisterResp {}
