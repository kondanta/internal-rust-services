# Collector

Main purpose of this component is to collect data from the remote queue and send it to the local queue.

## Configuration
.env file is used to configure the collector. The following environment variables are used:
- `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` - Hostname of the local trace collector (default: http://localhost:4317)
- `OTEL_TRACES_SAMPLER` - Sampler to use for traces (default: always_on)