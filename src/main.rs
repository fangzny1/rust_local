

use std::{fs::{ File, TryLockError::Error, canonicalize}, io::ErrorKind::NotFound, sync::Arc};
use clap::ValueHint::Url;
use tokio::{fs::ReadDir, io::{AsyncReadExt, AsyncWriteExt, join}};
use axum::{
    Router, body::{self, Body}, extract::{Multipart, Path, State, multipart}, handler, http::{HeaderMap, StatusCode, Uri, header}, response::{Html, IntoResponse, Response}, routing::{get, post}
};
use tokio_util::io::ReaderStream;
use clap::Parser;
#[derive(serde::Deserialize)]
pub struct Congfig
{
  share_dir:String,
  bind_addr:String,
  port:u32,
  inline_pdf:bool,
}

#[derive(Parser)]
struct Arg{
 
  reset:Option<String>

}
#[derive(Clone)]
struct AppState {
    share_dir: String,
    inline_pdf: bool,
}


async fn reset_check(final_conf:String,reset:&str)->Result<(), Box<dyn std::error::Error>>{
  match reset{
    "r"|"reset"=>{
      std::fs::write("./config.toml", &final_conf)?;
      println!("重置配置文件成功");
    }
    _=>{
      println!("本次启动没有传入参数");
    }
  }
Ok(())
}


#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>>{
    


    //默认配置
    let share_dir =String::from("/home/mnski/Share");
    let bind_addr=String::from("0.0.0.0");
    let port:u16 =8099;
    let inline_pdf=false;
    let final_conf=format!("share_dir =\"{share_dir}\" \n bind_addr=\"{bind_addr}\" \n port={port} \n inline_pdf={inline_pdf}");


    let check_conf=||{match std::fs::File::open("./config.toml"){
        Ok(_)=>true,
        Err(_)=>false
    }};
     if !check_conf(){
         std::fs::write("./config.toml", &final_conf)?;
     }

     //读取是否需要重置
     let reset =Arg::parse().reset;
     let reset =match reset {
       Some(v)=>v,
       None=>String::from("nope"),
     };
    
     reset_check(final_conf, &reset).await?;

     if check_conf(){
        let data_conf =std::fs::read_to_string("./config.toml")?;
        let decode_data =toml::from_str::<Congfig>(&data_conf)?;
  
        let bind_addr =decode_data.bind_addr;
        let port =decode_data.port.to_string();
        let appState_config=AppState{
          inline_pdf:decode_data.inline_pdf,
          share_dir:decode_data.share_dir,
        };
        let inline_pdf=Arc::from(decode_data.inline_pdf);
         let app =Router::new()
            .route("/", get(||async{"Hello world" }))
            .route("/file/{*id}", get(file))
            .route("/file/", get(nope_id))
            .route("/upload/{*dir}", post(upload))
            .with_state(appState_config)
         
            ;
        let addr_plus_port=format!("{bind_addr}:{port}");
        let listener =tokio::net::TcpListener::bind(&addr_plus_port).await?;
         println!("服务已启动,ip:{addr_plus_port},是否开启部分文件内浏览器预览：{}",&inline_pdf.to_string());
        axum::serve(listener, app).await?;
     }  
     else{
        panic!("尝试手动添加配置出现错误，请检查配置文件是否存在");
        
     }
    Ok(())
}
async fn nope_id(State(share_dir): State<AppState>) ->  Result<impl IntoResponse, StatusCode> {
    file(Path(String::from("")), State(share_dir)).await
}


async fn file(Path(id): Path<String>, State(share_dir): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    //接受用户传来文件路径id
    let user_file =&id;
    let root= share_dir.share_dir;
    let inline_pdf =share_dir.inline_pdf;
    //确认用户是否需要预览pdf等而不是下载

    //拼成完整的路径
    let  user_path=std::path::Path::new(&root).join(user_file);
    
    //路径"归一化"，防止..等跳出设定的目录
    let final_user_path =canonicalize(user_path).map_err(|_|StatusCode::NOT_FOUND)?;
   

 


    if final_user_path.is_dir(){
         let mut target_file= tokio::fs::read_dir(final_user_path).await.map_err(|_|StatusCode::NOT_FOUND)? ;
             let mut string_push:String =String::new();
               //获取文件路径下文件列表
              while  let Some(entry)=target_file.next_entry().await.map_err(|_|StatusCode::NOT_FOUND)? {
               let path_string =entry.path().display().to_string();
              let file_name =entry.file_name();
              let name_str = file_name.to_str().unwrap_or("未知文件名");
                
              let final_path_to_html=match !user_file.is_empty(){
                 true=>{
                     format!(" <a href=\"/file/{user_file}/{name_str}\">{name_str}</a>")
                     }
                    false=>{
                       format!(" <a href=\"/file/{user_file}{name_str}\">{name_str}</a>")
                    }
                     }      ;
                string_push.push_str(&final_path_to_html);
              string_push.push_str("\n");
            }
   
    return  Ok(Html(string_push).into_response());
    };
    if final_user_path.is_file(){
         
            let  target_file= tokio::fs::File::open(&final_user_path).await.map_err(|_|StatusCode::NOT_FOUND)? ;
            let file_option =final_user_path.extension().and_then(|e|e.to_str());
            let ext =match file_option{
              Some(v)=>v,
              None=>{
                "txt"
              }
            };
             return Ok(download_processor(target_file,final_user_path.file_name().unwrap().display().to_string(),ext,inline_pdf).await.into_response());
        // 获取包含路径的完整 PathBuf
       
    };
  Err(StatusCode::NOT_FOUND) 
}
async  fn download_processor(path:tokio::fs::File,name:String,ext:&str,inline_pdf:bool) ->impl IntoResponse{
   let stream =tokio_util::io::ReaderStream::new(path);
   let body =Body::from_stream(stream);




 let header=if inline_pdf{
  if ext =="pdf" {
      let header=[(header::CONTENT_TYPE,"application/pdf; charset=utf-8".to_string()),
     (header::CONTENT_DISPOSITION, format!("inline; filename=\"{}\"", name)),
   ];
    
   header
  }
  else if ext =="jpg" {
         let header=[(header::CONTENT_TYPE,"image/jpeg; charset=utf-8".to_string()),
     (header::CONTENT_DISPOSITION, format!("inline; filename=\"{}\"", name)),
   ];
    
   header
  }
  else if ext =="png" {
        let header=[(header::CONTENT_TYPE,"image/png; charset=utf-8".to_string()),
     (header::CONTENT_DISPOSITION, format!("inline; filename=\"{}\"", name)),
   ];
    
   header
  }
  else {
      let header=[(header::CONTENT_TYPE,"text/plain; charset=utf-8".to_string()),
     (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", name)),
   ];
   
   header
  }
 }
 else{
     let header=[(header::CONTENT_TYPE,"text/plain; charset=utf-8".to_string()),
     (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", name)),
   ];
    
   header
 };
(header,body)

}


async fn upload(Path(dir):Path<String>,State(share_dir): State<AppState>,mut multipart:Multipart,)->Result<impl IntoResponse,StatusCode>{
  while let Some(field)=multipart.next_field().await.map_err(|_|{StatusCode::NOT_FOUND})?{
      let root =canonicalize(&share_dir.share_dir).map_err(|_|{StatusCode::NOT_FOUND})?;
      let dir =std::path::Path::new(&root).join(&dir);
      let dir =canonicalize(dir).map_err(|_|{StatusCode::NOT_FOUND})?;
      let file_name =field.file_name().unwrap_or("未命名").to_string();
      let data =field.bytes().await.map_err(|_|{StatusCode::BAD_REQUEST})?;
    
    let safe_name =file_name.replace(['/','\\'], "");
  println!("{}",&safe_name);
    let save_file =std::path::Path::new(&dir).join(&safe_name);
     
    tokio::fs::write(&save_file, data).await.map_err(|_|{StatusCode::INTERNAL_SERVER_ERROR})?;
  }   
Ok(StatusCode::OK)
}
