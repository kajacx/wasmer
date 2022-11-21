use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

use wasmer_vfs::AsyncReadExt;

mod binary_package;
mod exec;
mod module_cache;

pub(crate) use exec::SpawnedProcess;
use sha2::*;

pub use self::{
    binary_package::*,
    exec::{spawn_exec, spawn_exec_module},
    module_cache::ModuleCache,
};
use crate::{os::command::Commands, WasiRuntimeImplementation, WasiState};

#[derive(Debug, Clone)]
pub struct BinFactory {
    pub(crate) state: Arc<WasiState>,
    pub(crate) commands: Commands,
    runtime: Arc<dyn WasiRuntimeImplementation + Send + Sync + 'static>,
    pub(crate) cache: Arc<ModuleCache>,
    pub(crate) local: Arc<RwLock<HashMap<String, Option<BinaryPackage>>>>,
}

impl BinFactory {
    pub fn new(
        state: Arc<WasiState>,
        compiled_modules: Arc<ModuleCache>,
        runtime: Arc<dyn WasiRuntimeImplementation + Send + Sync + 'static>,
    ) -> BinFactory {
        BinFactory {
            state,
            commands: Commands::new_with_builtins(runtime.clone(), compiled_modules.clone()),
            runtime,
            cache: compiled_modules,
            local: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn runtime(&self) -> &dyn WasiRuntimeImplementation {
        self.runtime.deref()
    }

    pub fn set_binary(&self, name: &str, binary: BinaryPackage) {
        let mut cache = self.local.write().unwrap();
        cache.insert(name.to_string(), Some(binary));
    }

    pub async fn get_binary(&self, name: &str) -> Option<BinaryPackage> {
        let name = name.to_string();

        // Fast path
        {
            let cache = self.local.read().unwrap();
            if let Some(data) = cache.get(&name) {
                return data.clone();
            }
        }

        // Slow path
        let mut cache = self.local.write().unwrap();

        // Check the cache
        if let Some(data) = cache.get(&name) {
            return data.clone();
        }

        // Check the filesystem for the file
        if name.starts_with("/") {
            if let Ok(mut file) = self
                .state
                .fs_new_open_options()
                .read(true)
                .open(name.clone())
            {
                // Read the file
                let mut data = Vec::with_capacity(file.size() as usize);
                if let Ok(_) = file.read_to_end(&mut data).await {
                    let package_name = name.split("/").last().unwrap_or_else(|| name.as_str());
                    let data = BinaryPackage::new(package_name, data.into());
                    cache.insert(name, Some(data.clone()));
                    return Some(data);
                }
            }
        }

        // NAK
        cache.insert(name, None);
        return None;
    }
}

pub fn hash_of_binary(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::default();
    hasher.update(data.as_ref());
    let hash = hasher.finalize();
    hex::encode(&hash[..])
}