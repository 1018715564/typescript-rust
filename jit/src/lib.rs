use std::{
    collections::HashMap,
    error::Error,
    path::Path,
    sync::{Arc, RwLock}, time::SystemTime,
};

use llvm_sys::{
    execution_engine::LLVMLinkInMCJIT,
    target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget},
};
use sha2::Digest;

use typescript_ast::parser;

mod builder;
mod callbacks;
mod context;
mod error;
mod module;
mod value;
mod stdlib;

pub use module::Module;
pub use value::Value;

fn source_hash(source: &str) -> Vec<u8> {
    let mut sha = sha2::Sha256::new();

    sha.update(source);
    sha.finalize().to_vec()
}

pub struct Runtime {
    modules: RwLock<HashMap<Vec<u8>, Arc<Module>>>,
}

impl Runtime {
    pub fn new() -> Self {
        unsafe {
            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();
        }

        Self {
            modules: RwLock::new(HashMap::new()),
        }
    }

    pub fn load_file<P: AsRef<Path>>(&self, filename: P, save_ir: Option<String>) -> Result<Arc<Module>, Box<dyn Error>> {
        let all_start = SystemTime::now();
        let start = SystemTime::now();
        let source = std::fs::read_to_string(filename)?;
        let hash = source_hash(&source);
        let dur = start.elapsed().unwrap();
        log::info!("read time: {}.{:06}", dur.as_secs(), dur.subsec_micros());

        let opt_module = self.modules.read().unwrap().get(&hash).cloned();
        let ret = if let Some(module) = opt_module {
            Ok(module.clone())
        } else {
            let start = SystemTime::now();
            let ast_module = parser::source(&source)?;
            let dur = start.elapsed().unwrap();
            log::info!("parse time: {}.{:06}", dur.as_secs(), dur.subsec_micros());

            let start = SystemTime::now();
            let mut builder = builder::Builder::new();

            builder
                .id(hash.clone())
                .standard_library();

            if let Some(save_ir) = save_ir {
                builder.save_ir(&save_ir);
            }
            
            let module = builder.build(&ast_module)?;
            // let module = Arc::new(Module::from_ast(hash.clone(), ast_module, save_ir)?);
            let dur = start.elapsed().unwrap();
            log::info!("build time: {}.{:06}", dur.as_secs(), dur.subsec_micros());

            self.modules.write().unwrap().insert(hash, module.clone());

            let start = SystemTime::now();
            module.run();
            let dur = start.elapsed().unwrap();
            log::info!("run time: {}.{:06}", dur.as_secs(), dur.subsec_micros());

            Ok(module)
        };

        let dur = all_start.elapsed().unwrap();
        log::info!("overall time: {}.{:06}", dur.as_secs(), dur.subsec_micros());

        ret
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {}
}
