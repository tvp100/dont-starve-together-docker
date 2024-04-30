use std::{
    fs,
    path::{Path, PathBuf},
};

use actix_multipart::form::{
    tempfile::{TempFile, TempFileConfig},
    MultipartForm,
};
use actix_web::{
    error, get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use derive_more::{Display, Error};
use fs_extra::dir::create_all;
use ini::Ini;
use serde::{Deserialize, Serialize};

mod fs_tool;
mod zip_tool;

#[derive(Debug, Serialize, Deserialize)]
struct JsonObj {
    name: String,
    age: i32,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let file_path =
        "D:\\code\\dont-starve-together-docker\\dst-server-management\\tmp\\Cluster_1.zip";
    let to_path = "D:\\code\\dont-starve-together-docker\\tmp\\tounzip";
    let check_path = "D:\\code\\dont-starve-together-docker\\tmp\\tounzip\\Cluster_1";
    let result = zip_tool::extra_zip_file(file_path, to_path);
    // 解压后，解压的目录就一个，（或者多个），去检查对应目录下的文件，然后返回
    let check = fs_tool::check_dir_format_right(check_path).unwrap();
    format!("Hello {name}, result {result}! check: {check}")
}

#[post("/test")]
async fn do_post(item: web::Json<JsonObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {req:?}");
    println!("model: {item:?}");

    HttpResponse::Ok().json(item.0)
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadMsg<'a> {
    status: i32,
    msg: &'a str,
    upload_count: i32,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "save file error: {}", msg)]
struct SaveFileErr {
    msg: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for SaveFileErr {}

#[post("save_files")]
async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, SaveFileErr> {
    let file_len = form.files.len();

    let mut file_path = String::new();

    for f in form.files {
        let file_name = f.file_name.unwrap();
        if !file_name.ends_with(".zip") {
            return Err(SaveFileErr {
                msg: "file is not zip",
            });
        }
        let path: String = format!("./tmp/{}", file_name);
        file_path = path.to_owned();
        log::info!("saving to {path}");
        f.file.persist(path).unwrap();
    }

    let result = extra_dst_file(file_path).await.unwrap();

    let upload_msg = UploadMsg {
        status: 200,
        msg: "okkkkkkkk",
        upload_count: result,
    };

    Ok(HttpResponse::Ok().json(upload_msg))
}

#[derive(Debug, Serialize, Deserialize)]
struct ParseFileResult<'a> {
    status: usize,
    title: &'a str,
    password: &'a str,
    description: &'a str,
    game_mod: &'a str,
    player: usize,
}

#[derive(Debug, Display, Error, Serialize, Deserialize)]
#[display(fmt = "{}", msg)]
struct ParseFileError {
    msg: String,
}

async fn extra_dst_file<'a, P: AsRef<Path>>(
    path: P,
) -> Result<ParseFileResult<'a>, Box<dyn std::error::Error>> {
    let to_path = "./tmp_extra/";
    create_all(to_path, true)?;
    let result = zip_tool::extra_zip_file(path, to_path);

    let entries = fs::read_dir(to_path).unwrap();

    let dir_names: Vec<String> = entries
        .filter_map(|entry| entry.ok().map(|path| path.path()))
        .filter(|path| path.is_dir())
        .filter_map(|path| {
            path.file_name()
                .and_then(|s| s.to_str().map(|s| s.to_owned()))
        })
        .collect();

    // 目前都限制只能上传一个目录，所以这个循环只触发一次
    for dir_name in dir_names.iter() {
        let full_path = format!("{}{}", to_path, dir_name);
        let check_result = fs_tool::check_dir_format_right(&full_path);
        if let Ok(check_result) = check_result {
            if check_result == true {
                // 初步校验成功，读取配置信息
                let ini_file = Path::new(&full_path).join("cluster.ini");
                let i = Ini::load_from_file(&ini_file).unwrap();
                let player = i
                    .get_from(Some("GAMEPLAY"), "max_players")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                
                let i = Ini::load_from_file(&ini_file).unwrap();
                let password = i
                    .get_from(Some("NETWORK"), "cluster_password")
                    .unwrap_or("");
                let i = Ini::load_from_file(&ini_file).unwrap();
                let cluster_name = i
                    .get_from(Some("NETWORK"), "cluster_name")
                    .unwrap_or("");
                let i = Ini::load_from_file(&ini_file).unwrap();
                let cluster_description = i
                    .get_from(Some("NETWORK"), "cluster_description")
                    .unwrap_or("");
                let i = Ini::load_from_file(&ini_file).unwrap();
                let parsed: ParseFileResult<'a> = ParseFileResult {
                    status: 200,
                    title: cluster_name,
                    password: password,
                    description: cluster_description,
                    game_mod: "地上+洞穴",
                    player: player,
                };
                return Ok(parsed);
            } else {
                return Err(Box::new(ParseFileError {
                    msg: "123".to_owned(),
                }));
            }
        } else {
            return Err(Box::new(ParseFileError {
                msg: "123".to_owned(),
            }));
        }
    }

    println!("{:?}", dir_names);

    Ok(ParseFileResult {
        status: 500,
        title: "",
        password: "",
        description: "",
        game_mod: "",
        player: 0,
    })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(greet)
            .service(do_post)
            .service(save_files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
