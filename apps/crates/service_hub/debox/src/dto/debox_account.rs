//! DeBox账号管理

use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
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
    /// 返回所有数据
    pub all: Option<bool>,
    /// 账号IDs
    pub account_ids: Option<Vec<i32>>,
    /// 账号状态
    pub status: Option<bool>,
    /// 排序字段
    /// sorts: ["id:asc"]
    pub sorts: Option<Vec<String>>,
}

/// 排序字段
pub struct DeboxAccountSort(pub debox_account::Column, pub sea_orm::Order);

impl TryFrom<String> for DeboxAccountSort {
    type Error = sea_orm::DbErr;

    // sort: "id:asc"
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            return Err(sea_orm::DbErr::Custom("Invalid sort format".to_string()));
        }
        let column = match parts[0] {
            "user_id" => debox_account::Column::UserId,
            "created_at" => debox_account::Column::CreatedAt,
            "updated_at" => debox_account::Column::UpdatedAt,
            _ => debox_account::Column::Id,
        };
        let order = match parts[1] {
            "asc" => sea_orm::Order::Asc,
            "desc" => sea_orm::Order::Desc,
            _ => sea_orm::Order::Asc,
        };
        Ok(DeboxAccountSort(column, order))
    }
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
    /// 账号ID
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
    /// 应用唯一标识，在DeBox开放平台申请
    pub app_id: String,
    /// 开发者 API Key，在DeBox开放平台获取
    pub api_key: String,
    /// 开发者 App Secret，在DeBox开放平台获取
    pub app_secret: String,
    /// 登录授权, 有效期较短
    pub access_token: String,
    /// WEB登录授权
    pub web_token: String,
    /// DeBox 用户ID
    pub debox_user_id: String,
    /// 描述信息
    pub desc: String,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 添加DeBox账号 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeboxAccountResp {}

/// 更新DeBox账号信息 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountReq {
    /// 账号ID
    pub id: i32,
    /// 应用唯一标识，在DeBox开放平台申请
    pub app_id: String,
    /// 开发者 API Key，在DeBox开放平台获取
    pub api_key: String,
    /// 开发者 App Secret，在DeBox开放平台获取
    pub app_secret: String,
    /// 登录授权, 有效期较短
    pub access_token: String,
    /// WEB登录授权
    pub web_token: String,
    /// DeBox 用户ID
    pub debox_user_id: String,
    /// 描述信息
    pub desc: String,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新DeBox账号信息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountResp {}

/// 更新DeBox账号状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDeboxAccountStatusReq {
    /// 账号ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新DeBox账号状态 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeboxAccountStatusResp {}

/// 删除DeBox账号 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDeboxAccountReq {
    /// 账号ID
    pub id: i32,
}

/// 删除DeBox账号 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeboxAccountResp {}

/// 更新所有账户信息 请求体
#[derive(Default, Deserialize, Validate)]
pub struct UpdateAllAccountsInfoReq {
    /// 用户ID
    pub user_id: i32,
}

/// 更新所有账户信息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAllAccountsInfoResp {}

/// 更新账户信息 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateAccountInfoReq {
    /// 账号ID
    pub id: i32,
}

/// 更新账户信息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountInfoResp {}

/// 下载配置文件 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct DownloadConfigFileReq {
    /// 账号ID
    pub id: i32,
}

/// 上传配置文件 请求体
///
/// 单文件上传
#[derive(TryFromMultipart)]
pub struct UploadConfigFileReq {
    // The `unlimited arguments` means that this field will be limited to the
    // total size of the request body. If you want to limit the size of this
    // field to a specific value you can also specify a limit in bytes, like
    // '5MiB' or '1GiB' or 'unlimited'.
    #[form_data(limit = "5MiB")]
    pub file: FieldData<NamedTempFile>,

    // This field will be limited to the default size of 1MiB.
    pub author: String,
}

/// 上传配置文件 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadConfigFileResp {}
