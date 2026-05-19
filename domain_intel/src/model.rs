use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Domain {
    #[arg(short, long, default_value_t = String::from("google.com"))]
    pub url: String,
}

#[derive(Debug)]
pub struct DomainInfo {
    pub domain: String,
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
    pub ttl: Option<u32>,
    pub cname: Vec<String>,
    pub mx: Vec<String>,
    pub txt: Vec<String>,
    pub ns: Vec<String>,
}
