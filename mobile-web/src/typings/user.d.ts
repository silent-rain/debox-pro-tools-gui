// User

// 获取用户个人信息
export interface ProfileResp {
  id: number; // 用户ID
  username: string; // 用户名称
  gender: number; // 性别
  age: number; // 年龄
  date_birth: string; // 出生日期
  avatar: string; // 头像URL
}
