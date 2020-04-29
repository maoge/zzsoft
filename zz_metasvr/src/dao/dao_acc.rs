pub struct DaoAcc{} 

impl DaoAcc {

    pub async fn acc_insert(acc: &ZZAccount, pg_client: &ClientWrapper) -> RetResponse {

        let stmt: tokio_postgres::Statement = pg_client
                .prepare("INSERT INTO zz_account\
                              (acc_id, acc_name, phone_num, mail, passwd, create_time)\
                          VALUES($1, $2, $3, $4, $5, $6)").await.unwrap();
        
        let mut mix_pass = String::from(&acc.acc_name);
        mix_pass.push_str(PASSWD_MIX);
        mix_pass.push_str(&acc.passwd.as_str());

        let key: [u8; 32] = [65; 32];
        let iv: [u8; 16] = [97; 16];
        let encrypted_data: Vec<u8> = aes_wrapper::encrypt(mix_pass.as_bytes(), &key, &iv).ok().unwrap();
        
        let reply = pg_client.query(&stmt, 
            &[&acc.acc_id, &acc.acc_name, &acc.phone_num, 
            &acc.mail, &encrypted_data.as_slice(), &acc.create_time]).await;
        let mut ret = RetResponse::new();
        match reply {
            Err(e) => {
                ret.ret_code = CODE_NOK;
                ret.ret_info = INFO_DB_ERR.to_string();
                error!("pg caught error: {:?}", e);
            },
            Ok(_) => {
                ret.ret_code = CODE_OK;
                ret.ret_info = ACC_CREATE_SUCC.to_string();
            },
        }

        info!("{}, {:?}", ACC_CREATE_SUCC, acc);

        ret
    }

    pub async fn check_acc(acc: &ZZAccount, pg_client: &ClientWrapper) -> RetResponse {
        let mut ret = RetResponse::new();
        ret.ret_code = CODE_OK;

        if acc.passwd == "" && ret.ret_code == CODE_OK {
            ret.ret_code = CODE_NOK;
            ret.ret_info = ACC_PASSWD_NOT_ALLOW_EMPTY.to_string();
            error!("{}", ACC_PASSWD_NOT_ALLOW_EMPTY);
        }

        if acc.acc_name == "" && ret.ret_code == CODE_OK {
            ret.ret_code = CODE_NOK;
            ret.ret_info = ACC_NAME_NOT_ALLOW_EMPTY.to_string();
            error!("{}", ACC_NAME_NOT_ALLOW_EMPTY);
        }

        if acc.phone_num == "" && ret.ret_code == CODE_OK {
            ret.ret_code = CODE_NOK;
            ret.ret_info = ACC_PHONE_NOT_ALLOW_EMPTY.to_string();
            error!("{}", ACC_PHONE_NOT_ALLOW_EMPTY);
        }

        if acc.mail == "" && ret.ret_code == CODE_OK {
            ret.ret_code = CODE_NOK;
            ret.ret_info = ACC_MAIL_NOT_ALLOW_EMPTY.to_string();
            error!("{}", ACC_MAIL_NOT_ALLOW_EMPTY);
        }

        // check mail regexp: xxx@xxx.xxx
        let mail_regexp = Regex::new(r"^[.a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$").unwrap();
        if !mail_regexp.is_match(&acc.mail.as_str())
                && ret.ret_code == CODE_OK {
            ret.ret_code = CODE_NOK;
            ret.ret_info = ACC_MAIL_REGX_NOT_MATCH.to_string();
            error!("邮箱:{}正则校验不符合规则", &acc.mail);
        }
        
        // check phone_num regexp:1[3,5,7,8,9]xxxxxxxxx
        let mphone_regexp = Regex::new(r"^((\+86)|(86))?(1[3|5|7|8|9])\d{9}$").unwrap();
        if !mphone_regexp.is_match(&acc.phone_num.as_str())
                && ret.ret_code == CODE_OK {
            ret.ret_code = CODE_NOK;
            ret.ret_info = ACC_PHONE_REGX_NOT_MATCH.to_string();
            error!("手机号码:{}正则校验不符合规则", &acc.phone_num);
        }

        if ret.ret_code == CODE_OK {
            let stmt: tokio_postgres::Statement = pg_client
                .prepare("SELECT COUNT(*) cnt FROM zz_account where acc_name = $1").await.unwrap();
            
            let reply = pg_client.query(&stmt, &[&acc.acc_name]).await;
            match reply {
                Ok(rows) => {
                    let cnt: i64 = rows[0].get("cnt");
                    if cnt > 0 {
                        ret.ret_code = CODE_NOK;
                        ret.ret_info = ACC_NAME_EXISTS.to_string();
                        error!("{},{}", ACC_NAME_EXISTS, &acc.acc_name);
                    }
                },
                Err(e) => {
                    ret.ret_code = CODE_NOK;
                    ret.ret_info = INFO_DB_ERR.to_string();
                    error!("pg caught error: {:?}", e);
                },
            }
        }

        if ret.ret_code == CODE_OK {
            let stmt: tokio_postgres::Statement = pg_client
                .prepare("SELECT COUNT(*) cnt FROM zz_account where phone_num = $1").await.unwrap();
            let reply = pg_client.query(&stmt, &[&acc.phone_num]).await;
            match reply {
                Ok(rows) => {
                    let cnt: i64 = rows[0].get("cnt");
                    if cnt > 0 {
                        ret.ret_code = CODE_NOK;
                        ret.ret_info = ACC_PHONE_EXISTS.to_string();
                        error!("{},{}", ACC_PHONE_EXISTS, &acc.phone_num);
                    }
                },
                Err(e) => {
                    ret.ret_code = CODE_NOK;
                    ret.ret_info = INFO_DB_ERR.to_string();
                    error!("pg caught error: {:?}", e);
                },
            }
        }

        if ret.ret_code == CODE_OK {
            let stmt: tokio_postgres::Statement = pg_client
                .prepare("SELECT COUNT(*) cnt FROM zz_account where mail = $1").await.unwrap();
            let reply = pg_client.query(&stmt, &[&acc.mail]).await;
            match reply {
                Ok(rows) => {
                    let cnt: i64 = rows[0].get("cnt");
                    if cnt > 0 {
                        ret.ret_code = CODE_NOK;
                        ret.ret_info = ACC_MAIL_EXISTS.to_string();
                        error!("{},{}", ACC_MAIL_EXISTS, &acc.mail);
                    }
                },
                Err(e) => {
                    ret.ret_code = CODE_NOK;
                    ret.ret_info = INFO_DB_ERR.to_string();
                    error!("pg caught error: {:?}", e);
                },
            }
        }

        if ret.ret_code == CODE_OK {
            ret.ret_info = ACC_CHECK_SUCC.to_string();
        }
        
        ret
    }

    pub async fn acc_login(login_info: &LoginInfo,
        pg_client: &ClientWrapper, ret_response: &mut RetResponse) {

        let mut mix_pass = String::from(&login_info.acc_name);
        mix_pass.push_str(PASSWD_MIX);
        mix_pass.push_str(&login_info.passwd.as_str());
    
        let key: [u8; 32] = [65; 32];
        let iv: [u8; 16] = [97; 16];
        let encrypted_data: Vec<u8> = aes_wrapper::encrypt(mix_pass.as_bytes(), &key, &iv).ok().unwrap();

        // let stmt: tokio_postgres::Statement = pg_client
        //         .prepare("select encode(passwd,'hex') passwd \
        //                     from zz_account where acc_name=$1").await.unwrap();
        let stmt: tokio_postgres::Statement = pg_client
                    .prepare("select count(*) cnt \
                                from zz_account where acc_name=$1 and passwd=$2").await.unwrap();
        let reply = pg_client.query(&stmt, &[&login_info.acc_name, &encrypted_data.as_slice()]).await;
        match reply {
            Err(e) => {
                ret_response.ret_code = CODE_NOK;
                ret_response.ret_info = INFO_DB_ERR.to_string();
                error!("pg caught error: {:?}", e);
            },
            Ok(rows) => {
                if rows.len() > 0 {
                    // let passwd_db: &str = rows[0].get("passwd");
                    let cnt: i64 = rows[0].get("cnt");
                    if cnt > 0 {
                        ret_response.ret_code = CODE_OK;
                        ret_response.ret_info = ACC_LOGIN_SUCC.to_string();
                    } else {
                        ret_response.ret_code = CODE_NOK;
                        ret_response.ret_info = ACC_LOGIN_FAIL.to_string();
                    }
                }
            },
        }
    }

    pub async fn get_magic_key(login_info: &LoginInfo,
        redis_conn: &mut ConnectionWrapper) -> (bool, String) {

        let key = String::from(&login_info.acc_name);
        // key.push_str(SESSION_STATE);

        let mut magic_key = String::new();
        let mut ok = false;
        let reply: redis::RedisResult<String> = deadpool_redis::cmd("HGET")
            .arg(key.clone())
            .arg(MAGIC_KEY)
            .query_async::<String>(redis_conn)
            .await;
        match reply {
            Err(_) => {  },
            Ok(s) => {
                ok = true;
                magic_key = s;
            },
        }

        (ok, magic_key)
    }

    pub async fn save_login_state(login_info: &LoginInfo,
            redis_conn: &mut ConnectionWrapper, 
            ret_response: &mut RetResponse) {

        let key = String::from(&login_info.acc_name);
        let ts = Local::now().timestamp_millis();

        deadpool_redis::cmd("HMSET").arg(key.clone())
            .arg(MAGIC_KEY)
            .arg(ret_response.magic_key.clone())
            .arg(SESSION_CREATE)
            .arg(ts)
            .arg(LOGIN_TIME)
            .arg(ts)
            .execute_async(redis_conn)
            .await.unwrap();
        deadpool_redis::cmd("EXPIRE").arg(key.clone()).arg(LOGIN_TTL)
            .execute_async(redis_conn)
            .await.unwrap();

        let session = Session {
            acc_name: login_info.acc_name.clone(),
            magic_key: ret_response.magic_key.clone(),
            login_time: ts,
        };
        GlobalSession::get().lock().unwrap().add_session(&session);
    }

    pub async fn refresh_login_time(login_info: &LoginInfo,
            redis_conn: &mut ConnectionWrapper) {

        let key = String::from(&login_info.acc_name);
        let ts = Local::now().timestamp_millis();
        deadpool_redis::cmd("HSET").arg(key.clone()).arg(LOGIN_TIME)
            .arg(ts)
            .execute_async(redis_conn)
            .await.unwrap();

        GlobalSession::get().lock().unwrap().refresh_login_time(key, ts);
    }

}
