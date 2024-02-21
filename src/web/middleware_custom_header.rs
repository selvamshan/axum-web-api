use axum::Extension;




#[derive(Clone)]
pub struct MiddleWareMessage{
    pub message: String
}

pub async fn middleware_custom_header(
    Extension(middleware_message): Extension<MiddleWareMessage>
) -> String {
    middleware_message.message
}