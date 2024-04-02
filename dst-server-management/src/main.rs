use actix_multipart::form::{
    tempfile::{TempFile, TempFileConfig},
    MultipartForm,
};
use actix_web::{
    get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
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


#[post("save_files")]
async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())
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