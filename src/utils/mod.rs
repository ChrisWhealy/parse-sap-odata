use std::{
    cmp::max,
    collections::HashMap,
    env,
    fmt::Debug,
    fs::OpenOptions,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::{anyhow, Context};
use serde::{Deserialize, Deserializer};
use which::which;

pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Find longest keyname in hashmap
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn longest(m: &HashMap<&str, &str>) -> usize {
    m.iter().fold(0, |max_len, e| max(max_len, e.0.len()))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Append entity definition to output buffer
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn write_entity<T: Debug>(out_buf: &mut Vec<u8>, maybe_entity: Option<&Vec<T>>) {
    match maybe_entity {
        Some(entity) => {
            if entity.len() > 0 {
                for e in entity {
                    out_buf.append(&mut format!("{:#?}\n", e).as_bytes().to_vec());
                }
                out_buf.append(&mut vec![10]); // Add line feed
            }
        },
        None => {},
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Deserialize string to Boolean
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn de_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).or_else(|_| Ok(false))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Pass the generated source code through rustfmt
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn run_rustfmt(buffer: &Vec<u8>, metadata_file_name: &str) -> Result<Vec<u8>, anyhow::Error> {
    let rustfmt_path = which("rustfmt").with_context(|| "Cannot find `rustfmt` in the path.  Is it installed?")?;

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
        // For diagnostic purposes during development, write the failed source code to $OUT_DIR as file
        // "failed_<metadata_file_name>.rs"
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let failed_file_name = format!("failed_{}.rs", metadata_file_name);
        let gen_failed_path = Path::new(&out_dir).join(failed_file_name);

        let _dont_care = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&gen_failed_path)
            .unwrap()
            .write_all(&buffer);

        let rustfmt_err_out = std::str::from_utf8(&rustfmt_output.stderr).unwrap();

        Err(anyhow!("Syntax error in generated source code:\n{}", rustfmt_err_out))
    }
}
