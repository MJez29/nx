use ignore::WalkBuilder;
use ignore_files::IgnoreFile;
use std::path::Path;
use std::{fs, path::PathBuf};
use tracing::trace;
use watchexec_events::{Event, Tag};

pub(super) fn get_ignore_files<T: AsRef<str>>(root: T) -> (Vec<IgnoreFile>, Option<IgnoreFile>) {
    let root = root.as_ref();

    let mut walker = WalkBuilder::new(root);
    walker.hidden(false);
    walker.git_ignore(false);

    let node_folder = PathBuf::from(root).join("node_modules");
    walker.filter_entry(move |entry| !entry.path().starts_with(&node_folder));
    let gitignore_files = walker
        .build()
        .flatten()
        .filter(|result| result.path().ends_with(".gitignore"))
        .map(|result| {
            let path: PathBuf = result.path().into();
            let parent: PathBuf = path.parent().unwrap_or(&path).into();
            IgnoreFile {
                path,
                applies_in: Some(parent),
                applies_to: None,
            }
        })
        .collect();
    (gitignore_files, get_nx_ignore(root))
}

fn get_nx_ignore<P: AsRef<Path>>(origin: P) -> Option<IgnoreFile> {
    let nx_ignore_path = PathBuf::from(origin.as_ref()).join(".nxignore");
    if nx_ignore_path.exists() {
        Some(IgnoreFile {
            path: nx_ignore_path,
            applies_in: Some(origin.as_ref().into()),
            applies_to: None,
        })
    } else {
        None
    }
}

// /// Get only the root level folders to watch.
// /// These will not include git ignored folders
// pub(super) fn get_watch_directories<T: AsRef<str>>(root: T) -> Vec<PathBuf> {
//     let root = root.as_ref();
//
//     let mut walker = WalkBuilder::new(root);
//     walker.hidden(false);
//     walker.max_depth(Some(1));
//     walker.filter_entry(|entry| entry.path().is_dir());
//
//     walker
//         .build()
//         .flatten()
//         .map(|result| result.path().into())
//         .collect()
// }

pub(super) fn transform_event(watch_event: &Event) -> Option<Event> {
    if cfg!(linux) {
        let tags = watch_event
            .tags
            .clone()
            .into_iter()
            .map(|tag| match tag {
                Tag::Path { path, file_type } => {
                    trace!("canonicalizing {:?}", path);
                    let real_path = fs::canonicalize(&path).unwrap_or(path);
                    trace!("real path {:?}", real_path);
                    Tag::Path {
                        path: real_path,
                        file_type,
                    }
                }
                _ => tag,
            })
            .collect();

        Some(Event {
            tags,
            metadata: watch_event.metadata.clone(),
        })
    } else {
        None
    }
}
