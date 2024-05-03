use std::{env, path::PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

struct Repository {
    path: PathBuf,
}

impl Repository {
    fn try_new() -> Option<Self> {
        if !cfg!(feature = "build-force")
            && env::var("DEP_ORTOOLS_LIB").is_ok()
            && env::var("DEP_ORTOOLS_INCLUDE").is_ok()
        {
            None
        } else {
            Some(Self::download())
        }
    }

    fn download() -> Self {
        // Configure
        const PREFIX: &str = concat!(
            "or-tools-",
            env!("CARGO_PKG_VERSION_MAJOR"),
            ".",
            env!("CARGO_PKG_VERSION_MINOR"),
        );
        const URL: &str = concat!(
            "https://github.com/google/or-tools/archive/refs/tags/",
            "v",
            env!("CARGO_PKG_VERSION_MAJOR"),
            ".",
            env!("CARGO_PKG_VERSION_MINOR"),
            ".tar.gz",
        );

        let path = PathBuf::from(
            env::var("OUT_DIR").expect("failed to get environment variable: OUT_DIR"),
        );

        // Download source code
        let file = {
            let response = ::ureq::get(URL)
                .call()
                .expect("failed to download source code");

            if response.status() != 200 {
                let code = response.status_text();
                panic!("failed to download source code {URL:?}: status code {code}");
            }

            response.into_reader()
        };

        // Extract the download file
        let mut archive = Archive::new(GzDecoder::new(file));
        archive
            .entries()
            .expect("failed to get entries from downloaded file")
            .filter_map(Result::ok)
            .for_each(|mut entry| {
                if let Some(path) = entry
                    .path()
                    .ok()
                    .and_then(|p| p.strip_prefix(PREFIX).ok().map(|p| path.join(p)))
                {
                    entry.unpack(path).expect("failed to extract file");
                }
            });

        Self { path }
    }

    fn build(&self) -> Library {
        trait CmakeFlag {
            fn to_bool(&self) -> &str;
        }

        impl CmakeFlag for bool {
            fn to_bool(&self) -> &str {
                if *self {
                    "ON"
                } else {
                    "OFF"
                }
            }
        }

        // Configure
        let install_dir = self.path.join("install");

        let mut builder = ::cmake::Config::new(&self.path);
        builder
            .define("AUTOBUILD", true.to_bool())
            .define("BUILD_CXX", true.to_bool())
            .define("BUILD_CXX_DOC", false.to_bool())
            .define("BUILD_CXX_EXAMPLES", false.to_bool())
            .define("BUILD_CXX_SAMPLES", false.to_bool())
            .define("BUILD_DEPS", true.to_bool())
            .define("BUILD_DOC", false.to_bool())
            .define("BUILD_DOTNET", false.to_bool())
            .define("BUILD_EXAMPLES", false.to_bool())
            .define("BUILD_JAVA", false.to_bool())
            .define("BUILD_PYTHON", false.to_bool())
            .define("BUILD_SAMPLES", false.to_bool())
            .define("BUILD_SHARED_LIBS", true.to_bool())
            .define("BUILD_TESTING", false.to_bool())
            .define("CMAKE_INSTALL_PREFIX", &install_dir);

        // Configure Solvers
        builder.define("USE_COINOR", cfg!(feature = "solver-coinor").to_bool());
        builder.define("USE_CPLEX", cfg!(feature = "solver-cplex").to_bool());
        builder.define("USE_GLPK", cfg!(feature = "solver-glpk").to_bool());
        builder.define("USE_HIGHS", cfg!(feature = "solver-highs").to_bool());
        builder.define("USE_SCIP", cfg!(feature = "solver-scip").to_bool());
        builder.define("USE_XPRESS", cfg!(feature = "solver-xpress").to_bool());

        // Build
        builder.build();

        Library {
            bin: install_dir.join("bin"),
            include: install_dir.join("include"),
            lib: install_dir.join("lib"),
        }
    }
}

struct Library {
    bin: PathBuf,
    include: PathBuf,
    lib: PathBuf,
}

impl Library {
    fn link(&self) {
        println!("cargo:bin={}", self.bin.display());
        println!("cargo:include={}", self.include.display());
        println!("cargo:lib={}", self.lib.display());
        println!("cargo:rustc-flags=-L {}", self.lib.display());
        println!("cargo:rustc-link-lib=ortools");
        println!("cargo:rustc-link-lib=protobuf");
        println!("cargo:rustc-link-lib=protoc");
        println!("cargo:rustc-link-search=native={}", self.lib.display());
    }
}

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");

    if let Some(repo) = Repository::try_new() {
        let library = repo.build();
        library.link()
    }
}
