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
        .get("Content-Type")
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
    let resp = format!("Post successfully created! {}", post.get_id());
    Ok(json_response(json!({"status": 200, "response": resp})))
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
            match field.name().unwrap() {
                "media" => {},
                // If someone decides to send multiple Content
                // Fields, the last one is the only one
                // We use
                "content" => {
                    // "content" should ONLY be text.
                    content = match field.text().await {
                        Ok(t) => t,
                        Err(_e) => return Err(StratError::BadMulti),
                    };
                }
                _ => {
                    // This should be impossible
                    return Err(StratError::BadMulti)
                }
            }
            // Keep looping
            true
        }
        // If there is no field, we immediately escape.
        Ok(_f) => false,
        Err(e) => match e {
            multer::Error::FieldSizeExceeded { limit, field_name } if field_name.is_some() => {
                return Err(StratError::OversizedField(field_name.unwrap(), limit))
            }
            _ => {
                return Err(StratError::BadMulti);
            }
        },
    } {}
    // If someone decides they don't want
    // To have a post body
    // We reject the request
    if content.is_empty() {
        return Err(StratError::NeedsContent)
    }
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

    // Posts can only have up to 500 characters
    // So we refuse the content if it exceeds that
    if p.content.len() > 500 {
        return Err(StratError::OversizedField("content".to_owned(), 500));
    }

    // Get post, return Error if any.
    let mut post = match Post::get_by_id(&p.id) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    // Permission Check
    // TODO: Replace with proper permissions system!
    if post.get_owner() != user.get_id() {
        return Err(StratError::NoPermission);
    }
    
    // Edit
    post.edit(p.content);

    // Save
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

    // Get post
    let post = match Post::get_by_id(&p.id) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    // Permission Check
    // TODO: Replace with proper permissions system!
    if post.get_owner() != user.get_id() {
        return Err(StratError::NoPermission);
    }

    // Delete
    if let Some(e) = post.delete_post() {
        return Err(e);
    }

    Ok(json_response(
        json!({"status": 200, "response": "Post successfully deleted!"}),
    ))
}