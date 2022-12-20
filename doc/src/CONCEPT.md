# Concept

- VU：并发用户，现实系统中操作业务的用户
- TPS（Transaction Per Second）：每秒事务数
- RPS（Request Per Second）：每秒请求数，适合用于容量规划和作为限流管控的参考依据
- RT（Response Time）：响应时间，业务从客户端发起到客户端接受的时间

## Performance Metrics

### Grafana

you can find all dashboard in https://grafana.com/grafana/dashboards/

```shell
cd C:\Program Files\GrafanaLabs\grafana\bin
# grafana的服务已经安装在windows的servers中了，直接启动即可
# then open browser in http://localhost:3000/
```

### Prometheus

detail(https://prometheus.io/docs/prometheus/latest/getting_started/)

```shell
cd D:\project\business-project\share\performance-test\prometheus-2.37.4.windows-amd64\prometheus-2.37.4.windows-amd64
# just run as administrator
# then open browser in http://localhost:9090/
```

#### config

```yml
# my global config
global:
  scrape_interval: 1s # Set the scrape interval to every 15 seconds. Default is every 1 minute.
  evaluation_interval: 1s # Evaluate rules every 15 seconds. The default is every 1 minute.
  # scrape_timeout is set to the global default (10s).

# Alertmanager configuration
alerting:
  alertmanagers:
    - static_configs:
        - targets:
          # - alertmanager:9093

# Load rules once and periodically evaluate them according to the global 'evaluation_interval'.
rule_files:
  # - "first_rules.yml"
  # - "second_rules.yml"

# A scrape configuration containing exactly one endpoint to scrape:
# Here it's Prometheus itself.
scrape_configs:
  # The job name is added as a label `job=<job_name>` to any timeseries scraped from this config.
  - job_name: "prometheus"

    # metrics_path defaults to '/metrics'
    # scheme defaults to 'http'.

    static_configs:
      - targets: ["localhost:9090"]
  # Windows Exporter
  - job_name: "windows_exporter"
    static_configs:
      - targets: ["localhost:9182"]
  # Mysql Exporter
  - job_name: mysql
    static_configs:
        - targets:
            - localhost:3306
    relabel_configs:
        - source_labels: [__address__]
          target_label: __param_target
        - source_labels: [__param_target]
          target_label: instance
        - target_label: __address__
          # The mysqld_exporter host:port
          replacement: localhost:9104
  # Redis Exporter
  - job_name: redis_exporter
    static_configs:
    - targets: ['localhost:9121']
```

#### exporter

##### mysqld_exporter (https://github.com/prometheus/mysqld_exporter)

.mysql_exporter.cnf

```cnf
[client]
user=exporter
password=exporter
```

```shell
cd D:\project\business-project\share\performance-test\mysqld_exporter-0.14.0.windows-amd64\mysqld_exporter-0.14.0.windows-amd64
./mysqld_exporter.exe --config.my-cnf=".mysql_exporter.cnf"
```

##### redis_exporter(https://github.com/oliver006/redis_exporter)

##### windows_exporter

