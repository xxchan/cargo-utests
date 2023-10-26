use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut list = nextest_metadata::ListCommand::new();
    // list.add_arg("--all-targets");
    // test default is true for lib, bins, and tests. Do not check examples, benches, etc.
    list.add_args(["--lib", "--bins", "--tests"]);
    tracing::info!("running {:?}", list.cargo_command());
    let list_result = list.exec()?;
    let mut no_tests: HashMap<String, Vec<String>> = HashMap::new();
    for suite in list_result.rust_suites.values() {
        if suite.test_cases.is_empty() {
            no_tests
                .entry(suite.binary.package_id.to_string())
                .or_default()
                .push(suite.binary.kind.to_string());
        }
    }

    let mut metadata = cargo_metadata::MetadataCommand::new();
    metadata.no_deps();
    tracing::info!("running {:?}", metadata.cargo_command());
    let metadata_result = metadata.exec()?;

    let mut fail = false;
    for package in metadata_result.packages {
        let span = tracing::info_span!("package", %package.name);
        let _enter = span.enter();

        if !no_tests.contains_key(&package.id.to_string()) {
            tracing::debug!("all targets have test cases");
        }
        let no_test_kinds = no_tests
            .get(&package.id.to_string())
            .map(Clone::clone)
            .unwrap_or_default();

        for target in package.targets {
            if target.is_bench() || target.is_example() || target.is_custom_build() {
                continue;
            }

            let span = tracing::info_span!("target", %target.name, ?target.kind);
            let _enter = span.enter();

            if target.kind.len() != 1 {
                // When will this happen?
                tracing::error!("multiple target kinds");
                panic!("multiple target kinds");
            }

            for target_kind in target.kind {
                let is_test_set = target.test;
                let has_test = !no_test_kinds.contains(&target_kind);
                match (is_test_set, has_test) {
                    (true, true) => {
                        tracing::debug!("test=true and has test");
                    }
                    (true, false) => {
                        tracing::warn!(
                            "There are no test cases for this target. You can set `test=false`."
                        );
                    }
                    (false, true) => {
                        tracing::error!("[DANGEROURS] test=false but has test");
                        fail = true;
                    }
                    (false, false) => {
                        tracing::debug!("test=false and no test");
                    }
                }
            }
        }
    }

    if fail {
        anyhow::bail!(
            "There are targets with `test=false` but have test cases. You'd better fix it."
        );
    }

    Ok(())
}
