use std::ptr;

pub fn new_patch<const N: usize>(
    target: *mut u8,
    replacement: [u8; N],
) -> Patch<InactiveState<N>> {
    Patch {
        target,
        inner: InactiveState {
            replacement,
        }
    }
}

trait PatchState {}

#[derive(Debug)]
pub struct Patch<TState: PatchState> {
    target: *mut u8,
    inner: TState,
}

#[derive(Debug)]
pub struct InactiveState<const N: usize> {
    replacement: [u8; N],
}

impl<const N: usize> PatchState for InactiveState<N> {}

impl<const N: usize> Patch<InactiveState<N>> {
    /// Applies memory edit represented by this patch.
    pub fn apply(&mut self) -> Patch<ActiveState<N>> {
        let mut original = [0x0u8; N];
        unsafe {
            // Backup the original
            ptr::copy_nonoverlapping(
                self.target,
                original.as_mut_ptr(),
                N
            );

            // Copy in the provided bytes
            ptr::copy_nonoverlapping(
                self.inner.replacement.as_ptr(),
                self.target,
                N
            );

        }
    
        Patch {
            target: self.target,
            inner: ActiveState {
                replacement: self.inner.replacement,
                original,
            }
        }
    }
}

#[derive(Debug)]
pub struct ActiveState<const N: usize> {
    replacement: [u8; N],
    original: [u8; N],
}

impl<const N: usize> PatchState for ActiveState<N> {}

impl<const N: usize> Patch<ActiveState<N>> {
    /// Rolls back the memory edits made by this patch.
    pub fn rollback(&mut self) -> Patch<InactiveState<N>> {
        unsafe {
            ptr::copy_nonoverlapping(
                self.inner.original.as_ptr(),
                self.target,
                N
            );
        }

        Patch {
            target: self.target,
            inner: InactiveState {
                replacement: self.inner.replacement,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::util::patch;

    #[test]
    pub fn patch_works() {
        let mut target: [u8; 4] =  [0x12, 0x34, 0x56, 0x78];

        let mut patch = patch::new_patch(target.as_mut_ptr(), [0x99, 0x99]);
        assert_eq!([0x12, 0x34, 0x56, 0x78], target);

        let mut patch = patch.apply();
        assert_eq!([0x99, 0x99, 0x56, 0x78], target);

        let mut patch = patch.rollback();
        assert_eq!([0x12, 0x34, 0x56, 0x78], target);

        patch.apply();
        assert_eq!([0x99, 0x99, 0x56, 0x78], target);
    }
}
