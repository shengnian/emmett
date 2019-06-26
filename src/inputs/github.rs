// Specification:

/// Read events from github webhooks.
pub struct Github {
    /// If Secret is defined, we drop the events that don’t match. Otherwise, we’ll just add an invalid tag.
    pub drop_invalid: Option<bool>,

    /// The ip to listen on.
    pub ip: Option<String>,

    /// The port to listen on.
    pub port: Option<u64>,

    /// Your GitHub Secret Token for the webhook.
    pub secret_token: Option<String>,
}
