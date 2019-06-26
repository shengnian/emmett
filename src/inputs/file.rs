// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-file.html
use std::path::Path;

#[derive(Debug)]
/// Stream events from files, normally by tailing them in a manner similar to tail -0F but optionally reading them from the beginning.

/// Normally, logging will add a newline to the end of each line written. By default, each event is assumed to be one line and a line is taken to be the text before a newline character. If you would like to join multiple log lines into one event, you’ll want to use the multiline codec. The plugin loops between discovering new files and processing each discovered file. Discovered files have a lifecycle, they start off in the "watched" or "ignored" state. Other states in the lifecycle are: "active", "closed" and "unwatched"

/// By default, a window of 4095 files is used to limit the number of file handles in use. The processing phase has a number of stages:

///    - Checks whether "closed" or "ignored" files have changed in size since last time and if so puts them in the "watched" state.
///    - Selects enough "watched" files to fill the available space in the window, these files are made "active".
///    - The active files are opened and read, each file is read from the last known position to the end of current content (EOF) by default. 

/// In some cases it is useful to be able to control which files are read first, sorting, and whether files are read completely or banded/striped. Complete reading is all of file A then file B then file C and so on. Banded or striped reading is some of file A then file B then file C and so on looping around to file A again until all files are read. Banded reading is specified by changing file_chunk_count and perhaps file_chunk_size. Banding and sorting may be useful if you want some events from all files to appear in Kibana as early as possible.

/// The plugin has two modes of operation, Tail mode and Read mode.

/// # Tail mode

/// In this mode the plugin aims to track changing files and emit new content as it’s appended to each file. In this mode, files are seen as a never ending stream of content and EOF has no special significance. The plugin always assumes that there will be more content. When files are rotated, the smaller or zero size is detected, the current position is reset to zero and streaming continues. A delimiter must be seen before the accumulated characters can be emitted as a line.

/// # Read mode

/// In this mode the plugin treats each file as if it is content complete, that is, a finite stream of lines and now EOF is significant. A last delimiter is not needed because EOF means that the accumulated characters can be emitted as a line. Further, EOF here means that the file can be closed and put in the "unwatched" state - this automatically frees up space in the active window. This mode also makes it possible to process compressed files as they are content complete. Read mode also allows for an action to take place after processing the file completely.

/// In the past attempts to simulate a Read mode while still assuming infinite streams was not ideal and a dedicated Read mode is an improvement.
pub struct File {
    pub close_older: Option<String>,
    pub delimiter: Option<String>,
    pub discover_interval: Option<u64>,
    pub exclude: Option<Vec<String>>,
    pub file_chunk_count: Option<u64>,
    pub file_chunk_size: Option<u64>,
    pub file_completed_action: Option<String>,
    pub file_completed_log_path: Option<String>,
    pub file_sort_by: Option<String>,
    pub file_sort_direction: Option<String>,
    pub ignore_older: Option<u64>,
    pub max_open_files: Option<u64>,
    pub mode: Option<String>,
    pub path: Option<Vec<&'static Path>>,
    pub sincedb_clean_after: Option<String>,
    pub sincedb_path: Option<String>,
    pub sincedb_write_interval: Option<String>,
    pub start_position: Option<String>,
    pub start_interval: Option<String>,
}

impl File {
    fn new() -> Self {
        Self {
            close_older: Some("1 hour".to_string()),
            delimiter: Some("\n".to_string()),
            discover_interval: Some(15),
            exclude: None,
            file_chunk_count: Some(4_611_686_018_427_387_903),
            file_chunk_size: Some(32768),
            file_completed_action: Some("delete".to_string()),
            file_completed_log_path: None,
            file_sort_by: Some("last_modified".to_string()),
            file_sort_direction: Some("asc".to_string()),
            ignore_older: None,
            max_open_files: Some(4095),
            mode: Some("tail".to_string()),
            path: None,
            sincedb_clean_after: Some("2 weeks".to_string()),
            sincedb_path: Some("<path.data>/plugins/inputs/file".to_string()),
            sincedb_write_interval: Some("15 seconds".to_string()),
            start_position: Some("end".to_string()),
            start_interval: Some("1 second".to_string()),
        }
    }
}
