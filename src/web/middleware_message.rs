use axum::extract::State;

use super::SharedData;



pub async fn middleware_message(State(shared_data): State<SharedData>) -> String{
    shared_data.message
}