use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::common::ErrorContext;

pub struct ReadDirExt<Context, F, G> {
    map_dir: F,
    map_file: G,
    q: Vec<(Option<Context>, PathBuf)>,
    current: Option<(Option<Context>, fs::ReadDir)>,
}

impl<Context, Item, F, G> ReadDirExt<Context, F, G>
where
    F: FnMut(Option<&Context>, &Path) -> io::Result<MapDirResult<Context>>,
    G: FnMut(Option<&Context>, PathBuf) -> io::Result<Option<Item>>,
{
    pub fn new(dir: PathBuf, map_dir: F, map_file: G) -> Self {
        Self {
            map_dir,
            map_file,
            q: vec![(None, dir)],
            current: None,
        }
    }

    fn next_io_result(&mut self) -> io::Result<Option<Item>> {
        loop {
            if let Some((context, read_dir)) = self.current.as_mut() {
                if let Some(dir_entry) = read_dir.next() {
                    let path_buf = dir_entry?.path();
                    if path_buf.is_dir() {
                        let context =
                            (self.map_dir)(context.as_ref(), &path_buf).context_lazy(|| {
                                format!("`map_dir` returned error for path {path_buf:?}")
                            })?;
                        match context {
                            MapDirResult::Mark(new_context) => {
                                self.q.push((Some(new_context), path_buf))
                            }
                            MapDirResult::Clear => self.q.push((None, path_buf)),
                            MapDirResult::Ignore => {}
                        }
                    } else {
                        let error_context =
                            format!("`map_file` returned error for path {path_buf:?}");
                        let item =
                            (self.map_file)(context.as_ref(), path_buf).context(error_context);
                        if item.is_err() || item.as_ref().is_ok_and(|item| item.is_some()) {
                            return item;
                        }
                    }
                } else {
                    self.current = None;
                }
            } else if let Some((context, path_buf)) = self.q.pop() {
                let context = if let Some(context) = context {
                    MapDirResult::Mark(context)
                } else {
                    (self.map_dir)(None, &path_buf)?
                };
                match context {
                    MapDirResult::Mark(context) => {
                        self.current = Some((Some(context), Self::read_dir(&path_buf)?));
                    }
                    MapDirResult::Clear => {
                        self.current = Some((None, Self::read_dir(&path_buf)?));
                    }
                    MapDirResult::Ignore => {}
                }
            } else {
                return Ok(None);
            }
        }
    }

    fn read_dir(path: &Path) -> io::Result<fs::ReadDir> {
        fs::read_dir(path)
            .context_lazy(|| format!("`fs::read_dir()` returned error for path {path:?}"))
    }
}

impl<Context, Item, F, G> Iterator for ReadDirExt<Context, F, G>
where
    F: FnMut(Option<&Context>, &Path) -> io::Result<MapDirResult<Context>>,
    G: FnMut(Option<&Context>, PathBuf) -> io::Result<Option<Item>>,
{
    type Item = io::Result<Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_io_result() {
            Ok(value) => value.map(Ok),
            Err(err) => Some(Err(err)),
        }
    }
}

#[derive(Debug)]
pub enum MapDirResult<T> {
    Mark(T),
    Clear,
    Ignore,
}
