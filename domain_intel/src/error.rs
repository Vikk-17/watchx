use hickory_resolver::Resolver;

#[derive(Debug)]
enum DomainIntelError {
    NetworkError,
    DnsError,
    TlsError,
}

// implementing the From trait for ResolveError -> target output DomainIntelError
impl From<ResolveError> for DomainIntelError {
    fn from(_: ResolveError) -> Self {
        DomainIntelError::DnsError
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use hickory_resolver::error::ResolveError;

    #[test]
    fn test_resolve_error_conversion() {
        let resolve_error = ResolveError::from("some dns error");

        let app_error: DomainIntelError = resolve_error.into();

        assert_eq!(app_error, DomainIntelError::DnsError);
    }
}
