use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("uv@{}", version)])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn pip(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "uv", "pip", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn venv(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "uv", "venv", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn clean(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "uv", "clean", &args])?
        .stdout()?;
    Ok(stdout)
}
