/// This input plugin permits to retrieve metrics from remote Java applications using JMX. Every polling_frequency, it scans a folder containing json configuration files describing JVMs to monitor with metrics to retrieve. Then a pool of threads will retrieve metrics and create events.
pub struct Jmx {

}
