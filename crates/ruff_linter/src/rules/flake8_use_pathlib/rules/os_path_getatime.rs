use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, violation};

/// ## What it does
/// Checks for uses of `os.path.getatime`.
///
/// ## Why is this bad?
/// `pathlib` offers a high-level API for path manipulation, as compared to
/// the lower-level API offered by `os`.
///
/// When possible, using `Path` object methods such as `Path.stat()` can
/// improve readability over the `os` module's counterparts (e.g.,
/// `os.path.getatime()`).
///
/// Note that `os` functions may be preferable if performance is a concern,
/// e.g., in hot loops.
///
/// ## Examples
/// ```python
/// import os
///
/// os.path.getatime(__file__)
/// ```
///
/// Use instead:
/// ```python
/// from pathlib import Path
///
/// Path(__file__).stat().st_atime
/// ```
///
/// ## References
/// - [Python documentation: `Path.stat`](https://docs.python.org/3/library/pathlib.html#pathlib.Path.stat)
/// - [Python documentation: `os.path.getatime`](https://docs.python.org/3/library/os.path.html#os.path.getatime)
/// - [PEP 428 – The pathlib module – object-oriented filesystem paths](https://peps.python.org/pep-0428/)
/// - [Correspondence between `os` and `pathlib`](https://docs.python.org/3/library/pathlib.html#correspondence-to-tools-in-the-os-module)
/// - [Why you should be using pathlib](https://treyhunner.com/2018/12/why-you-should-be-using-pathlib/)
/// - [No really, pathlib is great](https://treyhunner.com/2019/01/no-really-pathlib-is-great/)
#[violation]
pub struct OsPathGetatime;

impl Violation for OsPathGetatime {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`os.path.getatime` should be replaced by `Path.stat().st_atime`".to_string()
    }
}
