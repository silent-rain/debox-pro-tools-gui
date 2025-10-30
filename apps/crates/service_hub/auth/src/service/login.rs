//! 登陆

use log::error;
use nject::injectable;

use axum_jwt::Claims;
use entity::user::user_base;
use err_code::{Error, ErrorMsg};

use user::{EmailDao, PhoneDao, UserBaseDao, enums::user_base::UserType};
use utils::crypto::sha2_256;

use crate::dto::login::{LoginReq, LoginResp};

/// 服务层
#[injectable]
pub struct LoginService {
    user_dao: UserBaseDao,
    email_dao: EmailDao,
    phone_dao: PhoneDao,
}

impl LoginService {
    /// 登陆
    pub async fn login(&self, req: LoginReq) -> Result<LoginResp, ErrorMsg> {
        // 检测手机号码或邮件用户是否存在
        let user = self.get_user(req.clone()).await?;
        // 检查用户是否被禁用
        if !user.status {
            error!("{} 用户已被禁用", user.id);
            return Err(Error::LoginUserDisableError.into_err_with_msg("用户已被禁用"));
        }

        // 密码加密
        let password_hash = sha2_256(&req.password);
        // 检测密码
        if user.password != password_hash {
            error!("{} 账号或密码错误", user.id);

            return Err(Error::LoginPasswordError.into_err_with_msg("账号或密码错误"));
        }

        // JWT
        let token = Claims::encode_token(user.id, user.username.clone()).map_err(|e| {
            error!("{} 生成Token失败, err: {:#?}", user.id, e);
            Error::AxumJwt(e).into_err_with_msg("生成Token失败")
        })?;

        // 返回Token
        Ok(LoginResp {
            user_id: user.id,
            username: user.username,
            avatar: user.avatar,
            token,
        })
    }

    /// 获取用户信息
    async fn get_user(&self, data: LoginReq) -> Result<user_base::Model, ErrorMsg> {
        let user_id = match data.user_type {
            UserType::Base => self.get_user_base(data).await?,
            UserType::Phone => self.get_user_phone(data).await?,
            UserType::Email => self.get_user_email(data).await?,
        };

        // 查询用户
        let result = self
            .user_dao
            .info(user_id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户不存在");
                Error::DbQueryEmptyError.into_err_with_msg("该用户不存在")
            })?;

        Ok(result)
    }

    /// 获取用户名用户
    async fn get_user_base(&self, req: LoginReq) -> Result<i32, ErrorMsg> {
        let username = match req.username.clone() {
            Some(v) => v,
            None => {
                return Err(Error::InvalidParameter(
                    "请求参数错误, 用户名或密码 不能为空".to_string(),
                )
                .into_err());
            }
        };

        let user = self
            .user_dao
            .info_by_username(username)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户名或密码不存在");
                Error::DbQueryEmptyError.into_err_with_msg("该用户名或密码不存在")
            })?;

        Ok(user.id)
    }

    /// 获取用户手机号
    async fn get_user_phone(&self, data: LoginReq) -> Result<i32, ErrorMsg> {
        let phone = match data.phone.clone() {
            Some(v) => v,
            None => {
                return Err(
                    Error::InvalidParameter("请求参数错误, phone 不能为空".to_string()).into_err(),
                );
            }
        };

        let user = self
            .phone_dao
            .info_by_phone(phone)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户手机号不存在");
                Error::DbQueryEmptyError.into_err_with_msg("该用户手机号不存在")
            })?;

        Ok(user.user_id)
    }

    /// 获取用户邮箱
    async fn get_user_email(&self, data: LoginReq) -> Result<i32, ErrorMsg> {
        let email = match data.email.clone() {
            Some(v) => v,
            None => {
                return Err(
                    Error::InvalidParameter("请求参数错误, email 不能为空".to_string()).into_err(),
                );
            }
        };

        let user = self
            .email_dao
            .info_by_email(email)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户邮箱不存在");
                Error::DbQueryEmptyError.into_err_with_msg("该用户邮箱不存在")
            })?;

        Ok(user.user_id)
    }
}
