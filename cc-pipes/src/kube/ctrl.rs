use serde::Serialize;
use tokio::process::Command;
use crate::{Result, GenericError};

pub async fn kexec(args: Vec<String>) -> Result<()> {
    debug!("kubectl {}", args.join(" "));
    let s = Command::new("kubectl").args(&args).status().await?;
    if !s.success() {
        bail!("Subprocess failure from kubectl: {}", s.code().unwrap_or(1001))
    }
    Ok(())
}

async fn kout(args: Vec<String>) -> Result<(String, bool)> {
    debug!("kubectl {}", args.join(" "));
    let s = Command::new("kubectl").args(&args).output().await?;
    let out: String = String::from_utf8_lossy(&s.stdout).into();
    let err: String = String::from_utf8_lossy(&s.stderr).to_string().trim().into();
    if !err.is_empty() {
        warn!("kubectl {} stderr: {}", args.join(" "), err);
    }
    // kubectl keeps returning opening and closing apostrophes - strip them:
    if out.len() > 2 && out.starts_with('\'') {
        let res = out.split('\'').collect::<Vec<_>>()[1];
        return Ok((res.trim().into(), s.status.success()));
    }
    Ok((out, s.status.success()))
}


/// CLI way to resolve kube context
///
/// Should only be used from main.
pub async fn current_context() -> Result<String> {
    let (mut res, _) = kout(vec!["config".into(), "current-context".into()])
        .await
        .map_err(|e| {
            error!("Failed to Get kubectl config current-context. Is kubectl installed?");
            e
        })?;
    let len = res.len();
    if res.ends_with('\n') {
        res.truncate(len - 1);
    }
    Ok(res)
}

pub async fn set_context(context: &str, args: Vec<String>) -> Result<String> {
    let mut arg_list = vec!["config".into(), "set-context".into(), context.into()];
    arg_list.extend_from_slice(&args);

    let (res, _) = kout(arg_list).await.map_err(|e| {
        error!("Failed to set kubectl config set-context. Is kubectl installed?");
        e
    })?;

    Ok(res)
}

pub async fn use_context(context: &str) -> Result<String> {
    let (res, _) = kout(vec!["config".into(), "use-context".into(), context.into()])
        .await
        .map_err(|e| {
            error!("Failed to set kubectl config use-context. Is kubectl installed?");
            e
        })?;

    Ok(res)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[tokio::test]
    async fn ctx_works() -> Result<()> {
        let ctx=current_context().await?;
        println!("{}", ctx);
        Ok(())
    }
}

