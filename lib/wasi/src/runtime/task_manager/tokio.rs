use std::pin::Pin;

use futures::Future;
#[cfg(feature = "sys-thread")]
use tokio::runtime::{Builder, Runtime};
use wasmer::{vm::VMMemory, Module, Store};

use crate::{WasiCallingId, WasiThreadError};

use super::{SpawnType, VirtualTaskManager};

/// A task manager that uses tokio to spawn tasks.
#[derive(Debug)]
pub struct TokioTaskManager {
    /// This is the tokio runtime used for ASYNC operations that is
    /// used for non-javascript environments
    runtime: std::sync::Arc<Runtime>,
}

impl Default for TokioTaskManager {
    fn default() -> Self {
        let runtime: std::sync::Arc<Runtime> =
            std::sync::Arc::new(Builder::new_current_thread().enable_all().build().unwrap());
        Self { runtime }
    }
}

impl VirtualTaskManager for TokioTaskManager {
    /// See [`VirtualTaskManager::sleep_now`].
    fn sleep_now(
        &self,
        _id: WasiCallingId,
        ms: u128,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>> {
        Box::pin(async move {
            if ms == 0 {
                tokio::task::yield_now().await;
            } else {
                tokio::time::sleep(std::time::Duration::from_millis(ms as u64)).await;
            }
        })
    }

    /// See [`VirtualTaskManager::task_shared`].
    fn task_shared(
        &self,
        task: Box<
            dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static,
        >,
    ) -> Result<(), WasiThreadError> {
        self.runtime.spawn(async move {
            let fut = task();
            fut.await
        });
        Ok(())
    }

    /// See [`VirtualTaskManager::block_on`].
    fn block_on_generic<'a>(&self, task: Pin<Box<dyn Future<Output = ()> + 'a>>) {
        let _guard = self.runtime.enter();
        self.runtime.block_on(async move {
            task.await;
        });
    }

    /// See [`VirtualTaskManager::enter`].
    fn enter<'a>(&'a self) -> Box<dyn std::any::Any + 'a> {
        Box::new(self.runtime.enter())
    }

    /// See [`VirtualTaskManager::enter`].
    fn task_wasm(
        &self,
        task: Box<dyn FnOnce(Store, Module, Option<VMMemory>) + Send + 'static>,
        store: Store,
        module: Module,
        spawn_type: SpawnType,
    ) -> Result<(), WasiThreadError> {
        use wasmer::vm::VMSharedMemory;

        let memory: Option<VMMemory> = match spawn_type {
            SpawnType::CreateWithType(mem) => Some(
                VMSharedMemory::new(&mem.ty, &mem.style)
                    .map_err(|err| {
                        tracing::error!("failed to create memory - {}", err);
                    })
                    .unwrap()
                    .into(),
            ),
            SpawnType::NewThread(mem) => Some(mem),
            SpawnType::Create => None,
        };

        std::thread::spawn(move || {
            // Invoke the callback
            task(store, module, memory);
        });
        Ok(())
    }

    /// See [`VirtualTaskManager::task_dedicated`].
    fn task_dedicated(
        &self,
        task: Box<dyn FnOnce() + Send + 'static>,
    ) -> Result<(), WasiThreadError> {
        std::thread::spawn(move || {
            task();
        });
        Ok(())
    }

    /// See [`VirtualTaskManager::task_dedicated_async`].
    fn task_dedicated_async(
        &self,
        task: Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + 'static>> + Send + 'static>,
    ) -> Result<(), WasiThreadError> {
        let runtime = self.runtime.clone();
        std::thread::spawn(move || {
            let fut = task();
            runtime.block_on(fut);
        });
        Ok(())
    }

    /// See [`VirtualTaskManager::thread_parallelism`].
    fn thread_parallelism(&self) -> Result<usize, WasiThreadError> {
        Ok(std::thread::available_parallelism()
            .map(|a| usize::from(a))
            .unwrap_or(8))
    }
}