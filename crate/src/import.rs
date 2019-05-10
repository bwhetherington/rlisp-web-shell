use rlisp::rlisp_interpreter::{context::Context, expression::Expression, util::Str};

trait IsUrl {
    fn is_url(&self) -> bool;
}

impl<T> IsUrl for T
where
    T: AsRef<str>,
{
    fn is_url(&self) -> bool {
        let s = self.as_ref();
        s.starts_with("http://") || s.starts_with("https://")
    }
}

pub fn import(args: &[Expression], ctx: &mut Context) -> Expression {
    #[derive(Debug)]
    enum ResolvedFile {
        Url(Str),
        Path(Str),
    }

    impl ResolvedFile {
        fn as_str(&self) -> &str {
            match self {
                ResolvedFile::Url(url) => url.as_ref(),
                ResolvedFile::Path(path) => path.as_ref(),
            }
        }

        fn to_str(&self) -> Str {
            match self {
                ResolvedFile::Url(url) => url.clone(),
                ResolvedFile::Path(path) => path.clone(),
            }
        }
    }

    fn resolve_file_path(file_name: impl ToString, ctx: &Context) -> ResolvedFile {
        // Check if we're dealing with http or local file
        let file_name = file_name.to_string();
        // let file_name = clean_file_path(file_name);
        let file_path = Path::new(&file_name);
        let file_path_str = file_path.to_string_lossy();
        if file_path.is_absolute() || file_path_str.starts_with("~") || file_path_str.is_url() {
            // Absolute path (or relative to home)
            // We don't need to do anything
            // Check if http or local file
            if file_path_str.is_url() {
                ResolvedFile::Url(file_path_str.as_ref().into())
            } else {
                let path = std::fs::canonicalize(file_path).unwrap_or_default();
                let new_file = path.to_string_lossy().into_owned();
                ResolvedFile::Path(new_file.into())
            }
        } else if file_path.is_relative() {
            // Relative path

            let cur_file = ctx.get_cur_file();
            if let Some(cur_file) = cur_file {
                // Extract path from current file
                let cur_file = cur_file.to_string();
                let path = std::path::Path::new(&cur_file);

                let local_file_name = if file_name.starts_with("./") {
                    &file_name[2..]
                } else {
                    &file_name
                };
                let path = path.with_file_name(local_file_name);
                let path_str = path.to_string_lossy();

                // Check if it is now a url or not
                if path_str.is_url() {
                    ResolvedFile::Url(path_str.as_ref().into())
                } else {
                    let path = std::fs::canonicalize(path).unwrap_or_default();
                    let new_file = path.to_string_lossy().into_owned();
                    ResolvedFile::Path(new_file.into())
                }
            } else {
                // Current file is not defined, default to absolute path
                let new_file = file_path.to_string_lossy().into_owned();
                ResolvedFile::Path(new_file.into())
            }
        } else {
            // We aren't ready for this yet, but it can be for repositories
            let new_file = file_path.to_string_lossy().into_owned();
            ResolvedFile::Path(new_file.into())
        }
    }

    match args {
        [Str(file_name)] => {
            // Resolve file name relative to current file name
            let new_file_name = resolve_file_path(file_name, ctx);

            // Check if we have read the file already
            // if ctx.has_read_file(&new_file_name) {
            //     Expression::default()
            // } else {
            let file_str = new_file_name.to_str();
            ctx.add_file(file_str);
            let res = match &new_file_name {
                ResolvedFile::Url(url) => load_http(url),
                ResolvedFile::Path(path) => load_path(path),
            };

            // let res = load_file(&new_file_name);
            let prev_file_name = ctx.get_cur_file();
            ctx.insert("__FILE__", new_file_name.as_str());
            let res = res.map(|ex| ex.eval(ctx)).unwrap_or_else(|e| {
                Error(Rc::new(Exception::custom(
                    14,
                    format!(
                        "could not read file: \"{}\", reason: {}",
                        file_name,
                        e.to_string().to_lowercase()
                    ),
                )))
            });
            if let Some(prev) = prev_file_name {
                ctx.insert("__FILE__", prev);
            }
            if res.is_exception() {
                res
            } else {
                Expression::default()
            }
        }
        xs => Error(Rc::new(Exception::arity(1, xs.len()))),
    }
}
