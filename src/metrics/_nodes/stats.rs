use elasticsearch::nodes::NodesStatsParts;

use super::responses::NodesResponse;

pub(crate) const SUBSYSTEM: &'static str = "nodes_stats";

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-nodes-stats.html
async fn metrics(exporter: &Exporter) -> Result<Vec<Metrics>, elasticsearch::Error> {
    let response = exporter
        .client()
        .nodes()
        .stats(NodesStatsParts::Metric(
            &exporter.options().path_parameters_for_subsystem(SUBSYSTEM),
        ))
        .fields(&exporter.options().query_fields_for_subsystem(SUBSYSTEM))
        .request_timeout(exporter.options().timeout_for_subsystem(SUBSYSTEM))
        .send()
        .await?;

    let values = response
        .json::<NodesResponse>()
        .await?
        .into_values(&exporter.metadata(), REMOVE_KEYS)
        .await;

    Ok(metric::from_values(values))
}

// NOTE:
// enabling adaptive_selection exposes metrics in nanoseconds, e.g.: "avg_response_time_ns": 196669342
const REMOVE_KEYS: &[&'static str] = &[
    "timestamp",
    "attributes",
    "cgroup",
    "adaptive_selection",
    "mapped - 'non-volatile memory'",
    "pipelines",
    "classes",
    "script",
];

crate::poll_metrics!();

#[tokio::test]
async fn test_nodes_stats() {
    use tokio::sync::RwLock;

    use crate::metadata::node_data::{NodeData, NodeDataMap};

    let stats: NodesResponse =
        serde_json::from_str(include_str!("../../tests/files/nodes_stats.json"))
            .expect("valid json");

    let expected_name: String = "m1-nodename.example.com".into();

    let mut metadata = NodeDataMap::new();
    let _ = metadata.insert(
        "U-WnGaTpRxucgde3miiDWw".into(),
        NodeData {
            name: expected_name.clone(),
            ..Default::default()
        },
    );
    let metadata = RwLock::new(metadata);

    let values = stats.into_values(&metadata, &["timestamp"]).await;
    assert!(!values.is_empty());
    // When keys to remove: "timestamp"
    assert!(values[0].get("timestamp").is_none());

    let value = values.last().unwrap().as_object().unwrap();
    assert_eq!(value.get("name").unwrap().as_str().unwrap(), expected_name);

    let stats: NodesResponse =
        serde_json::from_str(include_str!("../../tests/files/nodes_stats.json"))
            .expect("valid json");
    // When keys remove empty
    let values = stats.into_values(&metadata, &[]).await;
    assert!(!values.is_empty());

    let values = values[0].as_object().unwrap();
    assert_eq!(values.get("name").unwrap().as_str().unwrap(), expected_name);

    assert!(values.get("timestamp").is_some());
}
