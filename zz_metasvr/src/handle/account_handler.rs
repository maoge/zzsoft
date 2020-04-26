
#[allow(dead_code)]
fn account_handler_regist(cfg: &mut web::ServiceConfig) {
    cfg.service(acc_insert)
       .service(acc_login);
}

///* http method: post
///* params:
///*   acc_id      String optional,
///*   acc_name    String required,
///*   phone_num   String required,
///*   mail        String required,
///*   passwd      String required,
///*   create_time i64    optional,
#[post("/acc/acc_insert")]
async fn acc_insert(info: web::Json<ZZAccount>,
    global_pool: web::Data<GlobalPool>,) -> RetResponse {
    
    let pg_pool = &global_pool.pg_pool;
    let pg_client = pg_pool.get().await.unwrap();

    let acc: &ZZAccount = &info.into_inner();

    let acc_id = uuid::Uuid::new_v4().to_string();
    let ts = Local::now().timestamp_millis();

    let account = ZZAccount {
        acc_id: acc_id,
        acc_name: acc.acc_name.clone(),
        phone_num: acc.phone_num.clone(),
        mail: acc.mail.clone(),
        passwd: acc.passwd.clone(),
        create_time: ts,
    };

    let mut ret: RetResponse = DaoAcc::check_acc(&account, &pg_client).await;
    println!("check:{:?}", ret);
    if ret.ret_code == CODE_OK {
        ret = DaoAcc::acc_insert(&account, &pg_client).await;
    }

    ret
}

///* http method: post
///* params:
///*   acc_name    String required,
///*   passwd      String required,
#[post("/acc/login")]
async fn acc_login(info: web::Json<LoginInfo>,
    global_pool: web::Data<GlobalPool>,) -> RetResponse {
    
    let pg_pool = &global_pool.pg_pool;
    let pg_client = pg_pool.get().await.unwrap();

    let mut ret_response = RetResponse::new();

    let login_info: &LoginInfo = &info.into_inner();
    DaoAcc::acc_login(&login_info, &pg_client, &mut ret_response).await;
    if ret_response.ret_code == CODE_OK {
        let redis_pool = &global_pool.redis_pool;
        let mut redis_conn = redis_pool.get().await.unwrap();

        match DaoAcc::get_magic_key(&login_info, &mut redis_conn).await {
            (true, magic_key) => {
                ret_response.magic_key = magic_key;
                DaoAcc::refresh_login_time(&login_info, &mut redis_conn).await;
            },
            (false, _) => {
                ret_response.magic_key = uuid::Uuid::new_v4().to_string();
                DaoAcc::save_login_state(&login_info, &mut redis_conn, &mut ret_response).await;
            },
        }
    }

    ret_response
}

// TODO:
// check magic_key from GlobalSession, when metasvr restart with GlobalSession data lost,
// then check data in redis, and load to GlobalSession, it is called passive loading through request.

// eventbus: notify login event, in oder to sync native memory GlobalSession data
// 
