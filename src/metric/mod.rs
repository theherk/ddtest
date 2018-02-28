use dipstick::*;

lazy_static! {
    pub static ref METRICS: AppMetrics<Statsd> = app_metrics(to_statsd("localhost:8125").unwrap().with_namespace(&["bend", "api"]));
    pub static ref GET: AppCounter<Statsd> = METRICS.counter("GET /");
}
