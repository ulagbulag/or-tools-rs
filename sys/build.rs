use std::{env, fs, path::PathBuf};

struct Repository {
    path: PathBuf,
}

impl Default for Repository {
    fn default() -> Self {
        // Configure
        const URL: &str = "https://github.com/google/or-tools.git";
        const TAG: &str = concat!(
            "v",
            env!("CARGO_PKG_VERSION_MAJOR"),
            ".",
            env!("CARGO_PKG_VERSION_MINOR"),
        );

        let path = PathBuf::from(
            env::var("OUT_DIR").expect("failed to get environment variable: OUT_DIR"),
        );

        // Clone repository
        let repo = ::git2::Repository::open(&path).unwrap_or_else(|_| {
            if path.exists() {
                fs::remove_dir_all(&path).expect("failed to cleanup output directory");
            }
            ::git2::Repository::clone(URL, &path).expect("failed to download source code")
        });

        // Checkout
        let (object, reference) = repo.revparse_ext(TAG).expect("failed to get Object");

        repo.checkout_tree(&object, None)
            .expect("failed to checkout");

        match reference {
            // gref is an actual reference like branches or tags
            Some(gref) => repo.set_head(gref.name().unwrap()),
            // this is a commit, not a reference
            None => repo.set_head_detached(object.id()),
        }
        .expect("failed to set HEAD");

        Self { path }
    }
}

impl Repository {
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
    }
}

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");

    let repo = Repository::default();
    let library = repo.build();
    library.link()
}
