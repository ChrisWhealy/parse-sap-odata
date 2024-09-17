/// Include a module generated build the build script.
///
/// You must specify the module name: for example:
///
/// ```rust,ignore
/// parse_sap_odata::include_mod!("gwsample_basic");
/// parse_sap_odata::include_mod!("gwsample_basic_metadata");
/// ```
///
/// # Note:
/// **This macro assumes you have not modified the output directory of `parse-sap-odata`!**.
///
/// The default output directory is available from the [`OUT_DIR`](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts) environment variable.
/// ```
#[macro_export]
macro_rules! include_mod {
    ($mod_name: tt) => {
        include!(concat!(env!("OUT_DIR"), concat!("/", $mod_name, ".rs")));
    };
}
