#[macro_use]
extern crate napi_derive;
use std::fs::File;
use std::io::{BufReader, Write};
extern crate num_cpus;
use crossbeam_channel::unbounded;
use ignore::{WalkBuilder, WalkState::Continue};
use napi::bindgen_prelude::*;
use std::cmp::min;
use std::path::Path;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

#[cfg(all(
    target_arch = "x86_64",
    not(target_env = "musl"),
    not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

const DEFAULT_ZIP_EXTENSION: &str = "zip";
#[napi(object)]
pub struct Options {
    pub target: Option<String>,
}

pub struct UnZip {
    inner: ZipArchive<File>,
    target_path: String,
}

pub struct Zip {
    inner: ZipWriter<File>,
    entry_path: String,
}

#[napi]
impl Task for Zip {
    type Output = ();
    type JsValue = ();

    fn compute(&mut self) -> Result<Self::Output> {
        let options = FileOptions::default();

        let path = Path::new(&self.entry_path);
        if path.is_dir() {
            let (tx, rx) = unbounded();
            WalkBuilder::new(&self.entry_path)
                .threads(min(30, num_cpus::get()))
                .build_parallel()
                .run(|| {
                    let tx = tx.clone();
                    Box::new(move |dir_entry_result| {
                        if let Ok(dir_entry) = dir_entry_result {
                            tx.send(dir_entry.path().to_owned()).unwrap();
                        }
                        Continue
                    })
                });
            let zip_paths: Vec<_> = rx.try_iter().collect();
            for zip_path in zip_paths {
                let name_path = zip_path
                    .strip_prefix(path)
                    .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;
                let name = match name_path.to_str() {
                    Some(name) => name,
                    None => {
                        return Err(Error::new(
                            Status::GenericFailure,
                            "Error with the name of file".to_string(),
                        ))
                    }
                };
                if zip_path.is_file() {
                    self.inner
                        .start_file(name, options)
                        .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;
                    let f = File::open(zip_path)?;
                    let br = BufReader::new(f);

                    self.inner.write_all(&*br.buffer())?
                } else if !name.is_empty() {
                    self.inner
                        .add_directory(name, options)
                        .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;
                }
            }
        } else {
            let name = match path.file_name() {
                Some(name) => name.to_str().unwrap(),
                None => {
                    return Err(Error::new(
                        Status::GenericFailure,
                        "Error with the name of file".to_string(),
                    ))
                }
            };
            self.inner
                .start_file(name, options)
                .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;
            let f = File::open(path)?;
            let br = BufReader::new(f);
            self.inner.write_all(&*br.buffer())?
        };
        self.inner
            .finish()
            .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;
        Ok(())
    }

    fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }

    fn finally(&mut self, _: Env) -> Result<()> {
        Ok(())
    }
}

#[napi]
impl Task for UnZip {
    type Output = ();
    type JsValue = ();

    fn compute(&mut self) -> Result<Self::Output> {
        self.inner
            .extract(&self.target_path)
            .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
    }

    fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }

    fn finally(&mut self, _: Env) -> Result<()> {
        Ok(())
    }
}

#[napi]
#[allow(non_snake_case)]
pub fn unzip(
    entryPath: String,
    options: Option<Options>,
    signal: Option<AbortSignal>,
) -> Result<AsyncTask<UnZip>> {
    let fname = Path::new(&entryPath);
    let target = match &options {
        Some(option) => {
            if let Some(dir) = &option.target {
                std::path::Path::new(dir)
            } else {
                fname.parent().unwrap()
            }
        }
        None => fname.parent().unwrap(),
    };
    let file = File::open(fname)?;
    let zip = ZipArchive::new(file).map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!(
                "Error with read zip file {} : {}",
                &fname.to_string_lossy(),
                e
            ),
        )
    })?;
    let target_path = match target.to_str() {
        Some(dir) => dir.to_string(),
        None => {
            return Err(Error::new(
                Status::GenericFailure,
                "Error with read path".to_string(),
            ))
        }
    };

    let unzip = UnZip {
        inner: zip,
        target_path,
    };
    match signal {
        Some(s) => Ok(AsyncTask::with_signal(unzip, s)),
        None => Ok(AsyncTask::new(unzip)),
    }
}

#[napi]
#[allow(non_snake_case)]
pub fn zip(
    entryPath: String,
    options: Option<Options>,
    signal: Option<AbortSignal>,
) -> Result<AsyncTask<Zip>> {
    let fname = Path::new(&entryPath);
    let target = match &options {
        Some(option) => {
            if let Some(target) = &option.target {
                Path::new(target).to_path_buf()
            } else {
                fname.with_extension(DEFAULT_ZIP_EXTENSION)
            }
        }
        None => fname.with_extension(DEFAULT_ZIP_EXTENSION),
    };
    let file = File::create(&target).map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!(
                "Error with read zip file {} : {}",
                &fname.to_string_lossy(),
                e
            ),
        )
    })?;
    let inner = ZipWriter::new(file);
    let zip = Zip {
        inner,
        entry_path: entryPath,
    };
    match signal {
        Some(s) => Ok(AsyncTask::with_signal(zip, s)),
        None => Ok(AsyncTask::new(zip)),
    }
}
