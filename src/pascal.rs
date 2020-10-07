pub type integer = i32;
pub type real = f32;

// TODO: Implement this.
pub struct ranged_unsigned_integer<B, MIN, MAX>(PhantomData<B>, PhantomData<MIN>, PhantomData<MAX>);

pub(crate) struct IoTarget {
    input_target: Box<dyn Read>,
    output_target: Box<dyn Write>,
}

#[allow(unused_variables)]
pub(crate) trait PascalFile {
    fn io_target(&self) -> &IoTarget;
    fn io_target_mut(&mut self) -> &mut IoTarget;
    fn write_fmt(&mut self, args: fmt::Arguments) -> io::Result<()> {
        self.io_target_mut().output_target.write_fmt(args)
    }
}

impl Default for IoTarget {
    fn default() -> Self {
        IoTarget {
            input_target: Box::new(io::empty()),
            output_target: Box::new(io::sink()),
        }
    }
}

// TODO: Implement this.
pub struct packed_file_of_text_char(IoTarget);

impl Default for packed_file_of_text_char {
    fn default() -> Self {
        packed_file_of_text_char(IoTarget::default())
    }
}

impl PascalFile for packed_file_of_text_char {
    fn io_target(&self) -> &IoTarget {
        &self.0
    }

    fn io_target_mut(&mut self) -> &mut IoTarget {
        &mut self.0
    }
}

// TODO: Implement this.
pub struct packed_file_of<T>(IoTarget, PhantomData<T>);

impl<T> Default for packed_file_of<T> {
    fn default() -> Self {
        packed_file_of(IoTarget::default(), PhantomData)
    }
}

impl<T> PascalFile for packed_file_of<T> {
    fn io_target(&self) -> &IoTarget {
        &self.0
    }

    fn io_target_mut(&mut self) -> &mut IoTarget {
        &mut self.0
    }
}

// TODO: Implement this.
pub struct file_of<T>(IoTarget, PhantomData<T>);

impl<T> Default for file_of<T> {
    fn default() -> Self {
        file_of(IoTarget::default(), PhantomData)
    }
}

impl<T> PascalFile for file_of<T> {
    fn io_target(&self) -> &IoTarget {
        &self.0
    }

    fn io_target_mut(&mut self) -> &mut IoTarget {
        &mut self.0
    }
}

#[allow(unused_variables)]
pub(crate) fn write<F: PascalFile, T: Display>(file: &mut F, val: T) {
    write!(file, "{}", val).unwrap();
}

#[allow(unused_variables)]
pub(crate) fn write_ln<F: PascalFile, T: Display>(file: &mut F, val: T) {
    writeln!(file, "{}", val).unwrap();
}

#[allow(unused_variables)]
pub(crate) fn write_ln_noargs<F: PascalFile>(file: &mut F) {
    writeln!(file, "").unwrap();
}

/// TODO: Implement this
#[allow(unused_variables)]
pub(crate) fn reset<F: PascalFile>(file: &mut F, path: &str, options: &str) {
    let new_input_target: Box<dyn Read> = if path == "TTY:" {
        Box::new(io::stdin())
    } else {
        unimplemented!()
    };
    file.io_target_mut().input_target = new_input_target;
}

/// TODO: Implement this
#[allow(unused_variables)]
pub(crate) fn rewrite<F: PascalFile>(file: &mut F, path: &str, options: &str) {
    let new_output_target: Box<dyn Write> = if path == "TTY:" {
        Box::new(io::stdout())
    } else {
        unimplemented!()
    };
    file.io_target_mut().output_target = new_output_target;
}

use core::fmt::{self, Display};
use core::marker::PhantomData;
use std::io::{self, Read, Write};
