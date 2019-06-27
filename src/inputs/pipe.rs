/// Stream events from a long running command pipe.
/// By default, each event is assumed to be one line. If you want to join lines, you’ll want to use the multiline codec.
pub struct Pipe {
    pub command: String
}
