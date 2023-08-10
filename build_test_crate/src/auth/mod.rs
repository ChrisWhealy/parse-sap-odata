use base64::{engine::general_purpose, Engine as _};
use std::{
    fs::File,
    io::BufReader,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

// ---------------------------------------------------------------------------------------------------------------------
// Fetch userid and password from .env file
// ---------------------------------------------------------------------------------------------------------------------
pub fn fetch_auth() -> Result<String, String> {
    let mut user = String::from("unknown");
    let mut pwd = String::from("unknown");

    // Try to obtain userid and password from environment variable file .env
    if let Ok(lines) = read_lines(".env") {
        for line in lines {
            match line {
                Ok(l) => {
                    if l.starts_with("SAP_USER") {
                        let (_, u) = l.split_at(l.find("=").unwrap() + 1);
                        user = u.to_owned();
                    }
                    if l.starts_with("SAP_PASSWORD") {
                        let (_, p) = l.split_at(l.find("=").unwrap() + 1);
                        pwd = p.to_owned();
                    }
                }
                Err(_) => (),
            }
        }
    }

    if user.eq("unknown") || pwd.eq("unknown") {
        Err("SAP userid and/or password missing from .env file".to_owned())
    } else {
        Ok(general_purpose::STANDARD.encode(format!("{}:{}", user, pwd)))
    }
}
