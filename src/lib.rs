#![feature(unreachable)]

trait UnsafeUnwrap {
    type Inner;

    unsafe fn unwrap_unchecked(self) -> Self::Inner;
}

impl<T> UnsafeUnwrap for Option<T> {
    type Inner = T;

    unsafe fn unwrap_unchecked(self) -> T {
        if let Some(inner) = self {
            inner
        } else {
            ::std::mem::unreachable()
        }
    }
}

impl<T, E> UnsafeUnwrap for Result<T, E> {
    type Inner = T;

    unsafe fn unwrap_unchecked(self) -> T {
        if let Ok(inner) = self {
            inner
        } else {
            ::std::mem::unreachable()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnsafeUnwrap;

    #[test]
    fn option() {
        let opt = Some(1);

        assert_eq!(unsafe { opt.unwrap_unchecked() }, 1);
    }

    #[test]
    fn result() {
        let opt: Result<usize, ()> = Ok(1);

        assert_eq!(unsafe { opt.unwrap_unchecked() }, 1);
    }
}