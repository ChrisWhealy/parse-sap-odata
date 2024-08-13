use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::{anyhow, Context};
use check_keyword::CheckKeyword;
use convert_case::Case;
use serde::{Deserialize, Deserializer};
use which::which;

pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize string to Boolean
pub fn de_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).or_else(|_| Ok(false))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize space-delimited string to list
pub fn de_str_to_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(" ").map(|fmt| String::from(fmt)).collect())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Case conversion
pub fn to_pascal_case(odata_name: &str) -> String {
    convert_case::Casing::to_case(&odata_name, Case::Pascal)
}

pub fn to_upper_camel_case(odata_type_name: &str) -> String {
    convert_case::Casing::to_case(&odata_type_name, Case::UpperCamel)
}

pub fn to_snake_case(odata_type_name: &str) -> String {
    convert_case::Casing::to_case(&odata_type_name, Case::Snake)
}

pub fn odata_name_to_rust_safe_name(odata_name: &str) -> String {
    CheckKeyword::into_safe(to_snake_case(odata_name))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Pass the generated source code through `rustfmt`
///
/// If `rustfmt` finds any errors, the source code is still written to `OUT_DIR`, but with `failed_` prefixed to the
/// filename
pub fn run_rustfmt(buffer: &Vec<u8>, file_name: &str) -> Result<Vec<u8>, anyhow::Error> {
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
        let failed_file_name = format!("failed_{}", file_name);
        let gen_failed_path = Path::new(&out_dir).join(failed_file_name);

        let _dont_care = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&gen_failed_path)?
            .write_all(&buffer);

        let rustfmt_err_out = std::str::from_utf8(&rustfmt_output.stderr)?;

        Err(anyhow!("Syntax error in generated source code:\n{}", rustfmt_err_out))
    }
}
