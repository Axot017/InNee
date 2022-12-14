#[macro_export]
macro_rules! define_port {
    ($id:ident = $f:tt$(<$($gen:tt),+>)?($($types:ident: $args:ty),*) -> $($out:tt)+) => {
        common_domain::paste::paste! {
            pub trait $id$(<$($gen),+>)?: $f($($args),*) -> Self::OutputFuture {
                type OutputFuture: std::future::Future<Output = $($out)+>;
            }

            impl<$($($gen),+,)?F, FUT> $id$(<$($gen),+>)? for F
            where
                F: $f($($args),*) -> FUT,
                FUT: std::future::Future<Output = $($out)+>,
            {
                type OutputFuture = FUT;
            }

            common_domain::lazy_static::lazy_static! {
                static ref [<$id:snake:upper>]: common_domain::tokio::sync::Mutex<()> = common_domain::tokio::sync::Mutex::new(());
            }

            pub async fn [<$id:snake _lock>]() -> common_domain::tokio::sync::MutexGuard<'static, ()> {
                [<$id:snake:upper>].lock().await
            }

            #[mockall::automock]
            pub mod [<$id:snake>] {
                use super::*;

                pub async fn call$(<$($gen),+>)?($([<_ $types>]: $args),*) -> $($out)+ {
                    unimplemented!()
                }
            }
        }
    };
}
