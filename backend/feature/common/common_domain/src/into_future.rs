use std::future::Ready;

pub trait IntoFuture: Sized {
    fn into_future(self) -> Ready<Self>;
}

impl<T> IntoFuture for T {
    fn into_future(self) -> Ready<Self> {
        std::future::ready(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_into_future() {
        let x = 1;
        let y = x.into_future();
        assert_eq!(y.await, 1);
    }
}
