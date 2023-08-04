include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

fn main() {
    let bp: BusinessPartner = Default::default();
    println!("{:#?}", bp);
}
