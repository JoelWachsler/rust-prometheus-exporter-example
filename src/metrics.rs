use prometheus::{Gauge, Opts, Registry, TextEncoder};

use crate::config::Config;

pub async fn update_and_get_metrics(config: &Config) -> anyhow::Result<String> {
    let r = Registry::new();

    for path_to_watch in &config.paths_to_watch {
        let dir_size = fs_extra::dir::get_size(&path_to_watch.path)?;

        let mut labels = path_to_watch.labels_as_map();
        labels.insert("path".into(), path_to_watch.path.clone());

        let gauge_opts = Opts::new("test_gauge", "test gauge help").const_labels(labels);

        let gauge = Gauge::with_opts(gauge_opts).unwrap();
        gauge.set(dir_size as f64);

        r.register(Box::new(gauge))?;
    }

    let encoder = TextEncoder::new();
    let metric_families = r.gather();
    let res = encoder.encode_to_string(&metric_families)?;

    Ok(res)
}
