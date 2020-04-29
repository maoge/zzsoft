
#[derive(Debug, Deserialize)]
pub struct GlobalZZConf {
    metasvr_conf: MetaSvr,
    db_pg_conf: PgConfig,
    redis_conf: Redis,
}

#[derive(Debug, Deserialize)]
pub struct MetaSvr {
    bind_address: String,
    keep_alive: usize,
    client_timeout: u64,
    shutdown_timeout: u64,
    maxconn: usize,
    maxconnrate: usize,
    workers: usize,
    ssl: bool,
}

#[derive(Debug, Deserialize)]
pub struct PgConfig {
    host: String,
    port: usize,
    user: String,
    password: String,
    dbname: String,
    max_size: u32,
    min_idle: u32,
    idle_timeout: u64,
    max_lifetime: u64,
    connection_timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    host: String,
    port: u16,
    passwd: String,
    max_size: u32,
    min_idle: u32,
    idle_timeout: u64,
    max_lifetime: u64,
    connection_timeout: u64,
}

impl GlobalZZConf {

    pub fn get_zz_conf() -> Arc<Mutex<GlobalZZConf>> {
        static mut G_ZZ_CONF: Option<Arc<Mutex<GlobalZZConf>>> = None;

        unsafe {
            G_ZZ_CONF.get_or_insert_with(|| {
                info!("init global zz_conf ......"); // do once
                let mut str_val = String::new();
                let conf_file = match env::var("ZZ_CONF") {
                    Err(_) => {
                        String::from("etc/zz_metasvr.toml")
                    },
                    Ok(s) => {
                        s
                    },
                };

                match File::open(conf_file) {
                    Ok(f) => {
                        let mut file = f;
                        match file.read_to_string(&mut str_val) {
                            Ok(s) => s,
                            Err(e) => {
                                error!("Error Reading file: {}", e);
                                panic!(e);
                            },
                        };
                    },
                    Err(e) => {
                        error!("open zz_metasvr.toml caught error:{:?}", e);
                        panic!(e);
                    },
                }

                let zz_conf: GlobalZZConf = toml::from_str(&str_val).unwrap();

                Arc::new(Mutex::new(GlobalZZConf {
                    metasvr_conf: zz_conf.metasvr_conf,
                    db_pg_conf: zz_conf.db_pg_conf,
                    redis_conf: zz_conf.redis_conf,
                }))
            })
            .clone()
        }

    }

}
