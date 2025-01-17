use axum::routing::{post, Router};

use futures_util::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;

struct Transaction {
    f1: (),
    f2: (),
}

pub(crate) struct SplitAddress(());

async fn handler_inner(ctx: Context) {
    let transactions = vec![];

    // This causes the lifetime issue:
    let iter = transactions
        .iter()
        .map(|x: &Transaction| (x.f1, SplitAddress(x.f2)));

    // This works:
    // let iter: Vec<_> = transactions
    //     .iter()
    //     .map(|x: &Transaction| (x.f1, SplitAddress(x.f2)))
    //     .collect();

    ctx.0.streaming_work(iter.into_iter()).await;
}

async fn handler() -> Result<String, String> {
    let ctx = Context(Arc::new(Service()));
    handler_inner(ctx).await;
    Ok("ok".to_string())
}

// or comment out router
pub async fn router() -> Router {
    Router::new().route("/bla", post(handler))
}

#[derive(Clone)]
pub struct Context(Arc<Service>);

struct Service();

impl Service {
    pub(crate) async fn streaming_work(
        &self,
        data: impl Iterator<Item = ((), SplitAddress)>,
    ) -> HashMap<(), Vec<String>> {
        futures_util::stream::iter(data)
            .map(|_| async move { todo!() })
            .buffer_unordered(100)
            .filter_map(|x| async move { x })
            .collect()
            .await
    }
}
