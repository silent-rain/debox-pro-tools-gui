//! 登出

use axum_context::Context;

use log::info;
use nject::injectable;

use err_code::ErrorMsg;

/// 服务层
#[injectable]
pub struct Logoutervice {}

impl Logoutervice {
    /// 登出
    pub async fn logout(&self, ctx: Context) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();
        let username = ctx.get_user_name();

        info!("用户 {user_id} - {username} 发起登出请求");
        Ok(())
    }
}
