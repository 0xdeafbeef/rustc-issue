use futures_util::StreamExt;
use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
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

async fn handler() {
    let ctx = Context(Arc::new(Service()));
    handler_inner(ctx).await;
}

// or comment out router
pub async fn router() {
    let _: MethodRouter<()> = post(handler);
}

pub fn post<H, T, S>(handler: H) -> MethodRouter<S, Infallible>
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    let _ = handler;
    MethodRouter {
        _s: PhantomData,
        _e: PhantomData,
    }
}

pub trait Handler<T, S>: Clone + Send + Sync + Sized + 'static {
    type Future: Future<Output = ()> + Send + 'static;

    fn call(self, state: S) -> Self::Future;
}

impl<F, Fut, S> Handler<((),), S> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
    S: Clone + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn call(self, _state: S) -> Self::Future {
        Box::pin(async move { self().await })
    }
}

pub struct MethodRouter<S = (), E = Infallible> {
    _s: PhantomData<S>,
    _e: PhantomData<E>,
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
