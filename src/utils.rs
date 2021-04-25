use std::{
    ffi::OsStr,
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use httparse::Request;
use nom::IResult;

pub type HeaderArray<'a> = [httparse::Header<'a>; MAX_HEADER_SIZE];

pub const MAX_HEADER_SIZE: usize = 100;

pub trait RetUnit
where
    Self: Sized,
{
    #[inline(always)]
    fn ret_unit(self) {}
}

impl<T> RetUnit for T {}

// Try canonicalize path, we don't care about Windows' "prefix" and Unix "root_path".
// This function will only resolve parent dir and remove current dir cases
// for performance and simplicity. The purpose is to take prevent directory traversal.
#[inline]
pub fn path_canonicalize_soft<'p, P: AsRef<Path> + 'p>(path: &'p P) -> Vec<&'p OsStr> {
    path.as_ref()
        .components()
        .fold(Vec::new(), |mut acc, item| {
            match item {
                Component::Normal(path) => acc.push(path),
                Component::ParentDir => acc.pop().ret_unit(),
                _ => {}
            }
            acc
        })
}

pub fn parse_request_line(input: &str) -> IResult<&str, &str> {
    unimplemented!()
}
