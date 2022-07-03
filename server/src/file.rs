use axum::{
    body::{Bytes, StreamBody},
    extract::{BodyStream, Multipart, Path},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    BoxError,
};
use futures::{Stream, TryStreamExt};
use std::io;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::{ReaderStream, StreamReader};

use crate::session::SessionId;

const UPLOADS_DIRECTORY: &str = "files";

// *** Save file ***
pub async fn save_request_body(
    _session_id: SessionId,
    Path(file_name): Path<String>,
    body: BodyStream,
) -> Result<(), (StatusCode, String)> {
    stream_to_file(&file_name, body).await
}

// *** Save files ***
pub async fn upload_many_file(
    _session_id: SessionId,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(&file_name, field).await?;
    }

    Ok(StatusCode::OK)
}

// Get file

pub async fn get_file(Path(file_name): Path<String>) -> impl IntoResponse {
    tracing::debug!("Get file ");

    if !path_is_valid(&file_name) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    let path = std::path::Path::new(UPLOADS_DIRECTORY).join(&file_name);
    tracing::debug!("Trying to get path:  {:?}", path);

    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("text/toml; charset=utf-8").unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", file_name)).unwrap(),
    );

    // let headers = Headers([
    //     (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
    //     (
    //         header::CONTENT_DISPOSITION,
    //         "attachment; filename=\"Cargo.toml\"",
    //     ),
    // ]);

    Ok((headers, body))
}

// *** Internal  *** //

// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path conists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(&*path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}

// pub async fn get_file(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
//     // let files = sqlx::query!(
//     //     r#"select * from file i
//     //     inner join file2file ai on i.id = ai.file_id
//     //     where ai.file_id = $1"#,
//     //     id
//     // )
//     // .fetch_all(&pool)
//     // .await
//     // .expect("Failed to execute list_gallery query")
//     // .into_iter()
//     // .map(|row| Image {
//     //     id: row.id,
//     //     title: row.title,
//     //     description: row.description,
//     //     file_url: row.file_url,
//     //     preview_url: String::from(""),
//     // })
//     // .collect::<Vec<_>>();

//     let files: Vec<String> = Vec::new();

//     Ok(warp::reply::json(&files))
// }

// pub async fn create_file(
//     form: FormData, // new_file: Album,
//                     // pool: PgPool
// ) -> Result<impl warp::Reply, Infallible> {
//     println!("Createfile!");

//     let parts: Vec<Part> = form.try_collect().await.ok().expect("Failed to parse file");

//     for p in parts {
//         println!("Part: {:?}", p);
//         if p.name() == "file" {
//             let content_type = p.content_type();
//             let file_ending;
//             match content_type {
//                 Some(file_type) => match file_type {
//                     "application/pdf" => file_ending = "pdf",
//                     "image/png" => file_ending = "png",
//                     "image/jpg" => file_ending = "jpg",
//                     "image/jpeg" => file_ending = "jpg",
//                     v => {
//                         eprintln!("invalid file type found: {}", v);
//                         return Ok(StatusCode::BAD_REQUEST);
//                     }
//                 },
//                 None => {
//                     eprintln!("file type could not be determined");
//                     return Ok(StatusCode::BAD_REQUEST);
//                 }
//             }

//             let file_name: String = p.filename().unwrap().to_string();
//             println!("Got filename: {}", file_name);

//             let value = p
//                 .stream()
//                 .try_fold(Vec::new(), |mut vec, data| {
//                     vec.put(data);
//                     async move { Ok(vec) }
//                 })
//                 .await
//                 .ok()
//                 .unwrap();

//             println!("Start writing file");
//             tokio::fs::write(format!("./images/{}", file_name), &value)
//                 .await
//                 .unwrap();
//             println!("File writing done");

//             println!("Got fileending: {}", file_ending);
//         }
//     }

//     // log::debug!("create_todo: {:?}", new_file);

//     // let _rec = sqlx::query!(
//     //     r#"
//     //         INSERT INTO public.file
//     //         (title, description, file_url)
//     //         VALUES ($1, $2, $3)
//     //         RETURNING id
//     //     "#,
//     //     new_file.title,
//     //     new_file.description,
//     //     new_file.file_url,
//     // )
//     // .fetch_one(&pool)
//     // .await
//     // .expect("Failed to insert new TODO");

//     Ok(StatusCode::CREATED)
// }
