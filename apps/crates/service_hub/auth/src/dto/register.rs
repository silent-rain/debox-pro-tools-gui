//! 注册

use serde::{Deserialize, Serialize};
use validator::Validate;

use user::enums::user_base::UserType;

/// 注册用户
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct RegisterReq {
    /// 注册用户类型
    pub register_type: UserType,
    /// 手机号码
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 区块链钱包
    pub blockchain_wallet: Option<String>,

    /// 密码
    #[validate(length(min = 6, message = "密码至少需要6个字符"))]
    pub password: String,

    // ==== 基础信息 ====
    /// 用户名称
    #[validate(length(min = 5, max = 20, message = "用户名必须在5到20个字符之间"))]
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别(0:保密,1:女,2:男)
    /// Enum: [`entity::user::Gender`]
    #[validate(range(min = 0, max = 2, message = "性别(0:保密,1:女,2:男)"))]
    pub gender: i8,
    /// 年龄
    /// TODO 待定，可以自己计算出来
    #[validate(range(min = 18, max = 100, message = "年龄必须在18到100岁之间"))]
    pub age: Option<i32>,
    /// 出生日期
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,

    // ==== 防止恶意注册 ====
    /// 验证码ID
    #[serde(default)]
    pub captcha_id: String,
    /// 验证码
    #[serde(default)]
    pub captcha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResp {}
