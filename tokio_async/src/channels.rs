pub mod oneshot_channel;
pub mod mpsc_channel;
pub mod watch_channel;
pub mod broadcast_channel;

/*
mpsc: multi-producer, single-consumer channel. Many values can be sent.
oneshot: single-producer, single consumer channel. A single value can be sent.
broadcast: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
watch: multi-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.
 */
