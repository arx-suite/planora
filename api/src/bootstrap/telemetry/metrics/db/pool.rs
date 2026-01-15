use opentelemetry::{KeyValue, global};
use sqlx::PgPool;

pub fn init(pool: PgPool, pool_name: String) {
    let meter = global::meter("db.pool");

    meter
        .i64_observable_gauge("db.pool.size")
        .with_callback({
            let pool = pool.clone();
            let name = pool_name.clone();
            move |obs| {
                obs.observe(
                    pool.size() as i64,
                    &[KeyValue::new("pool", name.to_owned())],
                );
            }
        })
        .build();

    meter
        .i64_observable_gauge("db.pool.idle")
        .with_callback({
            move |obs| {
                obs.observe(
                    pool.num_idle() as i64,
                    &[KeyValue::new("pool", pool_name.clone())],
                );
            }
        })
        .build();
}
