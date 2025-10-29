//! DeBox账号管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::debox::debox_account;

/// 查询DeBox账号列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetDeboxAccountsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 查询DeBox账号列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeboxAccountsResp {
    pub data_list: Vec<debox_account::Model>,
    pub total: u64,
}

impl From<(Vec<debox_account::Model>, u64)> for GetDeboxAccountsResp {
    fn from((data_list, total): (Vec<debox_account::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询DeBox账号信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDeboxAccountReq {
    /// DeBox账号ID
    pub id: i32,
}

/// 查询DeBox账号信息 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetDeboxAccountResp {
    #[serde(flatten)]
    model: debox_account::Model,
}

impl From<debox_account::Model> for GetDeboxAccountResp {
    fn from(model: debox_account::Model) -> Self {
        Self { model }
    }
}

/// 添加DeBox账号 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct CreateDeboxAccountReq {
    // /// 用户ID
    // pub user_id: i32,
    // /// 开发者 API Key，在DeBox开放平台获取
    // pub api_key: String,
    // /// 开发者 App Secret，在DeBox开放平台获取
    // pub app_secret: String,
    // /// 登录授权, 有效期较短
    // pub access_token: String,
    // /// WEB登录授权
    // pub web_token: String,
    // /// DeBox 用户ID
    // pub debox_user_id: String,
    // /// 用户钱包地址
    // pub wallet_address: String,
    // /// ApiKey 状态(bool:无效,true:有效)
    // pub api_key_status: bool,
    // /// Access Token 状态(bool:无效,true:有效)
    // pub access_token_status: bool,
    // /// Web Token 状态(bool:无效,true:有效)
    // pub web_token_status: bool,
    // /// 描述信息
    // pub desc: Option<String>,
    // /// 状态(false:停用,true:正常)
    // pub status: bool,
    #[serde(flatten)]
    pub model: debox_account::Model,
}

// impl Deref for CreateDeboxAccountReq {
//     type Target = debox_account::Model;

//     fn deref(&self) -> &Self::Target {
//         &self.model
//     }
// }

/// 添加DeBox账号 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeboxAccountResp {}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountReq {
    // /// DeBox账号ID
    // pub id: i32,
    // /// 用户ID
    // pub user_id: i32,
    // /// 开发者 API Key，在DeBox开放平台获取
    // pub api_key: String,
    // /// 开发者 App Secret，在DeBox开放平台获取
    // pub app_secret: String,
    // /// 登录授权, 有效期较短
    // pub access_token: String,
    // /// WEB登录授权
    // pub web_token: String,
    // /// DeBox 用户ID
    // pub debox_user_id: String,
    // /// 用户钱包地址
    // pub wallet_address: String,
    // /// ApiKey 状态(bool:无效,true:有效)
    // pub api_key_status: bool,
    // /// Access Token 状态(bool:无效,true:有效)
    // pub access_token_status: bool,
    // /// Web Token 状态(bool:无效,true:有效)
    // pub web_token_status: bool,
    // /// 描述信息
    // pub desc: Option<String>,
    // /// 状态(false:停用,true:正常)
    // pub status: bool,
    #[serde(flatten)]
    pub model: debox_account::Model,
}

// impl Deref for UpdateDeboxAccountReq {
//     type Target = debox_account::Model;

//     fn deref(&self) -> &Self::Target {
//         &self.model
//     }
// }

// impl DerefMut for UpdateDeboxAccountReq {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.model
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountResp {}

/// 更新数据状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountStatusReq {
    /// DeBox账号ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新数据状态 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDeboxAccountReq {
    /// DeBox账号ID
    pub id: i32,
}

/// 删除数据 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeboxAccountResp {}
