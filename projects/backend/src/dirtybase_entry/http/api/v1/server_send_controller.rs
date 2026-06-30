use dirtybase_contract::prelude::IntoResponse;

pub async fn sse_handler() -> impl IntoResponse {
    dirtybase_realtime::sse::ServerSendEvent::new("global").await
}
