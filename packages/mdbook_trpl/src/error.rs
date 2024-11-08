pub struct Report(Box<dyn std::error::Error>);

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct Simple(String);

impl Report {
    pub fn simple(s: impl Into<String>) -> Report {
        Report(Box::new(Simple(s.into())))
    }
}

impl<E> From<E> for Report
where
    E: std::error::Error + 'static,
{
    fn from(value: E) -> Self {
        Report(Box::new(value))
    }
}

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.0)?;
        let mut current_err = self.0.as_ref();
        while let Some(source_err) = current_err.source() {
            writeln!(f, "Caused by: {:?}", source_err)?;
            current_err = source_err;
        }
        Ok(())
    }
}
