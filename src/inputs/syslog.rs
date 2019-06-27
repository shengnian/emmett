/// Read syslog messages as events over the network.
/// 
/// This input is a good choice if you already use syslog today. It is also a good choice if you want to receive logs from appliances and network devices where you cannot run your own log collector.
/// Of course, syslog is a very muddy term. By default, this input only supports RFC3164 syslog with some small modifications. However, some non-standard syslog formats can be read and parsed if a functional grok_pattern is provided. The date format is still only allowed to be RFC3164 style or ISO8601.
/// For more information see the RFC3164 page.
/// > Note: This input will start listeners on both TCP and UDP.
pub struct Syslog {

}
