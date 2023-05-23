use anyhow::{anyhow, Context};
use serde::{Deserialize, Deserializer};
use std::{
    io::Write,
    process::{Command, Stdio},
};

use std::str::FromStr;
use which::which;

pub mod parse_error;

pub fn de_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).or_else(|_| Ok(false))
}

pub fn strip_namespace(some_name: &str, namespace: &str) -> String {
    let prefix = format!("{}.", namespace);

    if some_name.starts_with(&prefix) {
        some_name.strip_prefix(&prefix).unwrap().to_string()
    } else {
        some_name.to_string()
    }
}

pub fn run_rustfmt(buffer: &Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let rustfmt_path =
        which("rustfmt").with_context(|| "Cannot find `rustfmt` in the path.  Is it installed?")?;

    let mut fmt_proc = Command::new(rustfmt_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to spawn `rustfmt` process")?;

    {
        let mut stdin = fmt_proc.stdin.take().unwrap();
        stdin.write(buffer.into_iter().as_slice())?;
    }

    let rustfmt_output = fmt_proc.wait_with_output()?;

    if rustfmt_output.status.success() {
        Ok(rustfmt_output.stdout)
    } else {
        let rustfmt_err_out = std::str::from_utf8(&rustfmt_output.stderr).unwrap();

        Err(anyhow!(
            "Syntax error in generated source code:\n{}",
            rustfmt_err_out
        ))
    }
}
