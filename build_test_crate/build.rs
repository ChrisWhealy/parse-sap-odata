use parse_sap_odata::utils::parse_odata::gen_src;

fn main() {
    gen_src("zeam_pm_measure_srv", "ZEAM_PM_MEASURE_SRV");
    gen_src("gwsample_basic", "GWSAMPLE_BASIC");
}
