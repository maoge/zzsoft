include!("lib.rs");

use chrono::prelude::Local;
// use actix_web::{App, HttpServer};
// use actix_http::KeepAlive;
use ntex::web;
// use ntex::time::Seconds;
// use ntex_util::time::types::Seconds;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

fn main() {
    init_logger();
    init_conf();
    init_singleton();

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
                format_args!("{:>4}", level),
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

fn init_singleton() {
    // GlobalSession::get();
}

#[ntex::main]
async fn init_http_server() -> std::io::Result<()> {
    let g_zzconf_holder = GlobalZZConf::get_zz_conf();
    let g_zzconf = g_zzconf_holder.lock().unwrap();
    let meta_conf = &g_zzconf.metasvr_conf;

    let http_server = web::HttpServer::new(move || {
        web::App::new()
            .configure(test_handler_regist)
        })
        .keep_alive(meta_conf.keep_alive)
        // .client_timeout(ntex_util::time::types::Seconds::new(meta_conf.client_timeout))
        // .shutdown_timeout(Seconds::new(meta_conf.shutdown_timeout))  // request timeout
        .maxconn(meta_conf.maxconn)
        .maxconnrate(meta_conf.maxconnrate)            // maxconnrate * workers <= maxconn
        .workers(meta_conf.workers);                   // equal to CPU cores is Ok

    if !meta_conf.ssl {
        info!("start metasvr: http://{}", meta_conf.bind_address);
        // use ? may catch error internal, process term with no info, so use unwrap
        http_server.bind(&meta_conf.bind_address).unwrap().run().await
    } else {
        // generate cert file:
        // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file("./cert/key.pem", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("./cert/cert.pem").unwrap();

        info!("start metasvr: https://{}", meta_conf.bind_address);
        http_server.bind_openssl(&meta_conf.bind_address, builder).unwrap().run().await
    }

}
