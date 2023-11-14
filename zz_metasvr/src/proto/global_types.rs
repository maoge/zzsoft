const LOGIN_TTL: i32 = 3600;
    
const CODE_OK: i32 = 0;
const CODE_NOK: i32 = -1;
const _INFO_OK: &str = "OK";
const INFO_DB_ERR: &str = "db error";
const _INFO_NETWORK_ERR: &str = "io network error";
    
const PASSWD_MIX: &str = "^^||";
// const SESSION_STATE: &str = ":session";
const MAGIC_KEY: &str = "magic_key";
const SESSION_CREATE: &str = "create_time";
const LOGIN_TIME: &str = "login_time";

const ACC_CREATE_SUCC: &str = "账户创建成功";
const ACC_PASSWD_NOT_ALLOW_EMPTY: &str = "密码不能为空";
const ACC_NAME_NOT_ALLOW_EMPTY: &str = "用户名不能为空";
const ACC_PHONE_NOT_ALLOW_EMPTY: &str = "手机号不能为空";
const ACC_MAIL_NOT_ALLOW_EMPTY: &str = "邮箱不能为空";
const ACC_MAIL_REGX_NOT_MATCH: &str = "邮箱正则校验不符合规则";
const ACC_PHONE_REGX_NOT_MATCH: &str = "手机号码正则校验不符合规则";
const ACC_NAME_EXISTS: &str = "用户名已存在";
const ACC_PHONE_EXISTS: &str = "手机号码已存在";
const ACC_MAIL_EXISTS: &str = "邮箱已存在";
const ACC_LOGIN_SUCC: &str = "账户密码认证通过";
const ACC_LOGIN_FAIL: &str = "账户密码认证未通过";
const ACC_CHECK_SUCC: &str = "账户信息校验通过";


#[derive(Debug, Serialize, Deserialize)]
pub struct RetResponse {
    pub ret_code: i32,
    pub ret_info: String,
    pub magic_key: String,
}
    
impl Responder for RetResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
    
impl RetResponse {
    pub fn new() -> Self {
        Self { ret_code: CODE_NOK, ret_info: "".to_string(), magic_key: "".to_string(), }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZZAccount {
    pub acc_id: String,
    pub acc_name: String,
    pub phone_num: String,
    pub mail: String,
    pub passwd: String,
    pub create_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub acc_name: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    // pub acc_id: String,
    pub acc_name: String,
    pub magic_key: String,
    pub login_time: i64,
}

