# Elasticsearch exporter [![Build Status](https://travis-ci.org/vinted/elasticsearch_exporter-rs.svg?branch=master)](https://travis-ci.org/vinted/elasticsearch_exporter-rs)

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![BSD-3 licensed][bsd-badge]][bsd-url]

[crates-badge]: https://img.shields.io/crates/v/elasticsearch_exporter.svg
[crates-url]: https://crates.io/crates/elasticsearch_exporter
[docs-badge]: https://docs.rs/elasticsearch_exporter/badge.svg
[docs-url]: https://docs.rs/elasticsearch_exporter
[bsd-badge]: https://img.shields.io/badge/license-bsd-3.svg
[bsd-url]: LICENSE

Prometheus Elasticsearch exporter capable of working with large clusters.

## Features

 - Skips if metric zero/empty metrics
 - Metric collection is decoupled from rendering `/metrics` page

## Options

```shell
$ curl -s localhost:9222
Vinted Elasticsearch exporter
elasticsearch_url: http://127.0.0.1:9200
elasticsearch_global_timeout: 30s
elasticsearch_skip_labels
 - cat_allocation: health,status
 - cat_fielddata: id
 - cat_indices: health,status
 - cat_nodeattrs: id
 - cat_nodes: health,status,pid
 - cat_plugins: id,description
 - cat_segments: health,status,checkpoint,prirep
 - cat_shards: health,status,checkpoint,prirep
 - cat_templates: composed_of
 - cat_thread_pool: node_id,ephemeral_node_id,pid
 - cat_transforms: health,status
 - cluster_stats: segment,patterns
elasticsearch_include_labels
 - cat_aliases: index,alias
 - cat_allocation: node
 - cat_fielddata: node,field
 - cat_health: shards
 - cat_indices: index
 - cat_nodeattrs: node,attr
 - cat_nodes: index,name,node_role
 - cat_pending_tasks: index
 - cat_plugins: name
 - cat_recovery: index,shard,stage,type
 - cat_repositories: index
 - cat_segments: index,shard
 - cat_shards: index,node,shard
 - cat_templates: name,index_patterns
 - cat_thread_pool: node_name,name,type
 - cat_transforms: index
 - cluster_health: status
 - nodes_usage: name,usage
elasticsearch_skip_metrics
 - cat_aliases: filter,routing_index,routing_search,is_write_index
 - cat_health: epoch,timestamp
 - cat_nodeattrs: pid
 - cat_recovery: start_time,start_time_millis,stop_time,stop_time_millis
 - cat_templates: order
 - nodes_usage: _nodes_total,_nodes_successful,timestamp,since
elasticsearch_cat_headers
 - cat_nodes: *
exporter_poll_default_interval: 5s
exporter_poll_intervals
 - cluster_health: 5s
exporter_histogram_buckets: [0.02, 0.04, 0.06, 0.08, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 4.0, 6.0, 8.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0] in seconds
exporter_skip_zero_metrics: true
exporter_metrics_switch
 - cat_health: true
 - cat_indices: true
```

### DEFAULT SKIP LABELS

```
cat_allocation=health,status
cat_fielddata=id
cat_indices=health,status
cat_nodeattrs=id
cat_nodes=health,status,pid
cat_plugins=id,description
cat_segments=health,status,checkpoint,prirep
cat_shards=health,status,checkpoint,prirep
cat_templates=composed_of
cat_thread_pool=node_id,ephemeral_node_id,pid
cat_transforms=health,status
```


### DEFAULT INCLUDE LABELS

```
cat_health=shards
cat_aliases=index,alias
cat_allocation=node
cat_fielddata=node,field
cat_indices=index
cat_nodeattrs=node,attr
cat_nodes=index,name,node_role
cat_pending_tasks=index
cat_plugins=name
cat_recovery=index,shard,stage,type
cat_repositories=index
cat_segments=index,shard
cat_shards=index,node,shard
cat_templates=name,index_patterns
cat_thread_pool=node_name,name,type
cat_transforms=index
```

### DEFAULT SKIP METRICS

```
cat_health=epoch,timestamp
cat_aliases=filter,routing_index,routing_search,is_write_index
cat_nodeattrs=pid
cat_recovery=start_time,start_time_millis,stop_time,stop_time_millis
cat_templates=order
```

## Development

To start:

```shell
cargo run --bin elasticsearch_exporter
```

To test:

```shell
cargo test
```

# License

BSD-3-Clause
