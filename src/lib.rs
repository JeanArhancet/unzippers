#[macro_use]
extern crate napi_derive;
use std::fs::File;

use napi::bindgen_prelude::*;
use std::path::Path;
use zip::ZipArchive;

#[cfg(all(
    not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),
    not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[napi(object)]
pub struct Options {
    pub target: Option<String>,
}

pub struct UnZip {
    inner: ZipArchive<File>,
    target_path: String,
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
                Path::new(dir)
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