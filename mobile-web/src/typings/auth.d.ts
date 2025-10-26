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
}
