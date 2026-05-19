use domain_intel::{lookup_domain, map_lookup_to_domain_info};
use clap::Parser;
use domain_intel::errors::AppError;
use domain_intel::model::Domain;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = Domain::parse();
    let domain: String = args.url;
    let lookup = lookup_domain(domain.clone()).await?;
    let info = map_lookup_to_domain_info(domain, lookup);
    println!("{:#?}", info);

    Ok(())

}
