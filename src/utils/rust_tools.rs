use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Context};
use which::which;

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
