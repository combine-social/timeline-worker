macro_rules! here {
    ($err:ident) => {
        format!("@{}#{}: {}", file!(), line!(), $err.to_string())
    };
}

pub(crate) use here;
