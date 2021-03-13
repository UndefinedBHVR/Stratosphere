use crate::{
    error::StratError,
    post::structure::Post,
    user::structure,
    util::{json_response, parse_body},
};
use hyper::{Body, Request, Response};
use multer::{Constraints, Multipart, SizeLimit};
use routerify::ext::RequestExt;

// Creates a post using Multipart Form Data
pub async fn create_post(req: Request<Body>) -> Result<Response<Body>, StratError> {
    let user = req.context::<structure::User>().unwrap();
    let boundary = req
        .headers()
        .get("CONTENT_TYPE")
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| multer::parse_boundary(ct).ok());

    let constraints = Constraints::new()
        // Only allow Content or Media
        .allowed_fields(vec!["content", "media"])
        .size_limit(
            SizeLimit::new()
                // Set 15mb as size limit for the whole stream body.
                .whole_stream(15 * 1024 * 1024)
                // Set 8mb as size limit for all fields.
                .per_field(8 * 1024 * 1024)
                // The post's content can only contain 500 characters.
                .for_field("content", 500),
        );
    if boundary.is_none() {
        return Err(StratError::BadMulti);
    }
    let body = req.into_body();
    let post = match parse_post(
        body,
        boundary.unwrap(),
        constraints,
        user.get_id().to_owned(),
    )
    .await
    {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    if let Some(e) = post.save_post() {
        return Err(e);
    }
    Ok(json_response(
        json!({"status": 200, "response": "Post successfully created!"}),
    ))
}

async fn parse_post(
    body: Body,
    boundary: String,
    constraints: Constraints,
    owner: String,
) -> Result<Post, StratError> {
    let mut multipart = Multipart::new_with_constraints(body, boundary, constraints);
    let mut content = String::new();
    // Hacky way to return my own Error Type while parsing through all fields.
    // Probably better way to do this, will improve soon(tm)
    while match multipart.next_field().await {
        Ok(f) if f.is_some() => {
            let field = f.unwrap();
            if let Some(ty) = field.content_type() {
                if ty.eq(&mime::TEXT_PLAIN) {
                    content = match field.text().await {
                        Ok(t) => t,
                        Err(_e) => return Err(StratError::BadMulti),
                    }
                }
            }
            true
        }
        Ok(_f) => false,
        Err(e) => match e {
            multer::Error::FieldSizeExceeded { limit, field_name } if field_name.is_some() => {
                return Err(StratError::OversizedField(field_name.unwrap(), limit))
            }
            _ => return Err(StratError::BadMulti),
        },
    } {}
    Ok(Post::new(content, owner, true))
}

pub async fn edit_post(mut req: Request<Body>) -> Result<Response<Body>, StratError> {
    let user = req.context::<structure::User>().unwrap();
    #[derive(Serialize, Deserialize)]
    struct PostEdit {
        id: String,
        content: String,
    }

    let p: PostEdit = match parse_body::<PostEdit>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };

    if p.content.len() > 500 {
        return Err(StratError::OversizedField("content".to_owned(), 500));
    }

    let mut post = match Post::get_by_id(&p.id) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    if post.get_owner() != user.get_id() {
        return Err(StratError::NoPermission);
    }

    post.edit(p.content);
    if let Some(e) = post.save_post() {
        return Err(e);
    }

    Ok(json_response(
        json!({"status": 200, "response": "Post successfully edited!"}),
    ))
}

pub async fn delete_post(mut req: Request<Body>) -> Result<Response<Body>, StratError> {
    let user = req.context::<structure::User>().unwrap();
    #[derive(Serialize, Deserialize)]
    struct PostDelete {
        id: String,
    }

    let p: PostDelete = match parse_body::<PostDelete>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };

    let post = match Post::get_by_id(&p.id) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    if post.get_owner() != user.get_id() {
        return Err(StratError::NoPermission);
    }

    if let Some(e) = post.delete_post() {
        return Err(e);
    }

    Ok(json_response(
        json!({"status": 200, "response": "Post successfully edited!"}),
    ))
}
