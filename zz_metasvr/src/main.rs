include!("lib.rs");

use actix_web::{App, HttpServer};

fn main() {
    init_logger();
    init_conf();

    init_http_server().unwrap();
}

fn init_logger() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let level = { buf.default_styled_level(record.level()) };

            writeln!(
                buf,
                "{} {} [{}:{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
                format_args!("{:>4}", level),                   //record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                record.file().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                &record.args()
            )
            
        })
        .init();

    info!("env_logger initialized.");
}

fn init_conf() {
    // init global zz conf
    GlobalZZConf::get_zz_conf();
}

#[actix_rt::main]
async fn init_http_server() -> std::io::Result<()> {

    let global_pool = GlobalPool::get();

    let g_zzconf_holder = GlobalZZConf::get_zz_conf();
    let g_zzconf = g_zzconf_holder.lock().unwrap();
    let meta_conf = &g_zzconf.metasvr_conf;

    let http_server = HttpServer::new(move || {
        App::new()
            .data(global_pool.clone())
            .configure(test_handler_regist)
            .configure(account_handler_regist)
        })
        .keep_alive(meta_conf.keep_alive)
        .client_timeout(meta_conf.client_timeout)
        .shutdown_timeout(meta_conf.shutdown_timeout)  // request timeout
        .maxconn(meta_conf.maxconn)
        .maxconnrate(meta_conf.maxconnrate)            // maxconnrate * workers <= maxconn
        .workers(meta_conf.workers);                   // equal to CPU cores is Ok
    if !meta_conf.ssl {
        info!("start metasvr: http://{}", meta_conf.bind_address);
        // use ? may catch error internal, process term with no info, so use unwrap
        http_server.bind(meta_conf.bind_address.clone()).unwrap().run().await
    } else {
        info!("start metasvr: https://{}", meta_conf.bind_address);
        // generate cert file:
        // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
        use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file("./cert/key.pem", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("./cert/cert.pem").unwrap();
        http_server.bind_openssl(&meta_conf.bind_address, builder).unwrap().run().await
    }

}
