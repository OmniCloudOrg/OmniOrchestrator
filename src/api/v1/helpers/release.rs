use rocket::data::Data;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::fs;

pub async fn release<'a>(
    app_id: String,
    release_version: String,
    content_type: &ContentType,
    data: Data<'a>,
) -> Result<Status, Status> {
    println!("Starting deploy handler");
    println!("Content-Type: {:?}", content_type);

    println!("Build started for app: {:#?}", app_id);

    let mut options = MultipartFormDataOptions::new();

    // Add multiple possible field names to help debug
    options
        .allowed_fields
        .push(MultipartFormDataField::file("media").size_limit(5 * 1024 * 1024 * 1024));
    options
        .allowed_fields
        .push(MultipartFormDataField::file("file").size_limit(5 * 1024 * 1024 * 1024));
    options
        .allowed_fields
        .push(MultipartFormDataField::file("upload").size_limit(5 * 1024 * 1024 * 1024));

    // Parse form data with detailed error handling
    let form_data = match MultipartFormData::parse(content_type, data, options).await {
        Ok(form) => {
            println!("Successfully parsed form data");
            form
        }
        Err(e) => {
            println!("Error parsing form data: {:?}", e);
            return Err(Status::new(400));
        }
    };

    // Print ALL available fields for debugging
    println!("Available fields in form_data:");
    println!("Raw fields: {:#?}", form_data.raw);
    println!("Text fields: {:#?}", form_data.texts);
    println!("Files: {:#?}", form_data.files);

    // Check each possible file field for compatibility with CLI and third-party tools
    for field_name in ["media", "file", "upload"] {
        // if the field is found to have data accept the file and save it
        if let Some(files) = form_data.files.get(field_name) {
            println!("Found files in field '{}': {:?}", field_name, files);

            // Check if the file is valid, if so, process it
            if let Some(file) = files.first() {
                println!("Processing file:");
                println!("  Path: {:?}", file.path);
                println!("  Filename: {:?}", file.file_name);
                println!("  Content-Type: {:?}", file.content_type);

                let mut release_version = release_version.clone();

                // Create App directory
                if release_version.is_empty() {
                    release_version = uuid::Uuid::new_v4().to_string();
                }

                // Handle the case where the directory does not exist
                match fs::create_dir_all(format!("./Apps/{}/{}", app_id, release_version)) {
                    Ok(_) => {
                        let dir = std::path::PathBuf::from(format!(
                            "./Apps/{}/{}",
                            app_id, release_version
                        ));
                        let canon_dir = dir.canonicalize().unwrap();
                        log::info!("Created Directory at {}", canon_dir.display())
                    }
                    Err(_) => {
                        return Err::<Status, Status>(Status::new(500));
                    }
                }

                // Copy file with size verification
                let source_size = fs::metadata(&file.path)
                    .map_err(|_| Err::<Status, Status>(Status::new(500)))
                    .unwrap()
                    .len();

                println!("Source file size: {} bytes", source_size);

                match fs::copy(
                    &file.path,
                    format!("./Apps/{}/{}/release.tar.gz", app_id, release_version),
                ) {
                    Ok(bytes_written) => {
                        println!("Successfully wrote {} bytes", bytes_written);
                        if bytes_written == source_size {
                            return Ok(Status::new(200));
                        } else {
                            return Err(Status::new(500));
                        }
                    }
                    Err(e) => {
                        println!("Error copying file: {:?}", e);
                        return Err(Status::new(500));
                    }
                }
            } else {
                println!("No valid file found in request");
                return Err(Status::new(500));
            }
        }
    }
    Ok::<Status, Status>(Status::new(200))
}
