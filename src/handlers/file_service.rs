use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{web, HttpResponse, Result};
use futures::StreamExt;
use std::io::Write;
use std::fs::File;

async fn save_file(field: &mut Field) -> std::io::Result<File> {
    let content_disposition = field.content_disposition().unwrap();
    let filename = content_disposition.get_filename().unwrap();
    let filepath = format!("./{}", sanitize_filename::sanitize(&filename));
    let mut file = web::block(|| std::fs::File::create(filepath)).await?;

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        file = web::block(move || file.write_all(&data).map(|_| file)).await?;
    }

    Ok(file)
}

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        match save_file(&mut field).await {
            Ok(_) => println!("File saved successfully."),
            Err(err) => eprintln!("Error saving file: {}", err),
        }
    }

    Ok(HttpResponse::Ok().into())
}