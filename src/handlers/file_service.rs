use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Result};
use futures::StreamExt;

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
    let upload_status = files::save_file(&mut payload, "file".to_string(),"filename.jpg".to_string()).await;

    match upload_status {
        Ok(true) => Ok(HttpResponse::Ok()
            .json("update_succeeded")),
        _ => Ok(HttpResponse::BadRequest()
            .json("update_failed")),
    }
}

pub mod files {
    use std::io::Write;
    use actix_multipart::Multipart;
    use actix_web::web;
    use futures::{StreamExt, TryStreamExt};
    use crate::errors::EveryError;

    pub async fn save_file(payload: &mut Multipart, file_path: String,file_name:String) -> Result<bool, EveryError> {
        // iterate over multipart stream
        while let Some(mut field) = payload.try_next().await? {
            let content_type = field.content_disposition().unwrap();
            // let filename = content_type.get_filename().unwrap();
            let filepath = format!("{}/{}",file_path,file_name);

            // File::create is a blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath)).await??;

            // Field in turn is a stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || {
                    f.write_all(&data).map(|_| f)
                })
                .await??;
            }
        }
        Ok(true)
    }

}
