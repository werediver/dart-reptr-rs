use std::{fmt::Display, time::Duration};

pub trait Counter<T> {
    fn count(&mut self, event: T);
}

pub mod event {
    use std::time::Duration;

    pub struct FileLoaded {
        pub size: usize,
        pub duration: Duration,
    }
    pub struct FileParsed {
        pub size: usize,
        pub utf8_validation_duration: Duration,
        pub parsing_duration: Duration,
    }
}

#[derive(Default, Debug)]
pub struct Stats {
    loaded_count: usize,
    loaded_size: usize,
    parsed_count: usize,
    parsed_size: usize,
    loading_duration: Duration,
    utf8_validation_duration: Duration,
    parsing_duration: Duration,
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MEG: f64 = 1024.0 * 1024.0;

        f.write_fmt(format_args!(
            "Loaded {:.2} MiB in {:.2} s, {:.2} MiB/s\n",
            self.loaded_size as f64 / MEG,
            self.loading_duration.as_secs_f64(),
            self.loaded_size as f64 / self.loading_duration.as_secs_f64() / MEG
        ))?;
        f.write_fmt(format_args!(
            "UTF-8 validation took {:.2} s, {:.2} MiB/s\n",
            self.utf8_validation_duration.as_secs_f64(),
            self.loaded_size as f64 / self.utf8_validation_duration.as_secs_f64() / MEG
        ))?;
        f.write_fmt(format_args!(
            "Parsed {:.2} MiB in {:.2} s, {:.2} MiB/s\n",
            self.parsed_size as f64 / MEG,
            self.parsing_duration.as_secs_f64(),
            self.parsed_size as f64 / self.parsing_duration.as_secs_f64() / MEG
        ))?;
        f.write_fmt(format_args!(
            "Parsed {} out of {} files, {:.4}",
            self.parsed_count,
            self.loaded_count,
            self.parsed_count as f64 / self.loaded_count as f64
        ))
    }
}

impl Counter<event::FileLoaded> for Stats {
    fn count(&mut self, event: event::FileLoaded) {
        self.loaded_count += 1;
        self.loaded_size += event.size;
        self.loading_duration += event.duration;
    }
}

impl Counter<event::FileParsed> for Stats {
    fn count(&mut self, event: event::FileParsed) {
        self.parsed_count += 1;
        self.parsed_size += event.size;
        self.utf8_validation_duration += event.utf8_validation_duration;
        self.parsing_duration += event.parsing_duration;
    }
}
