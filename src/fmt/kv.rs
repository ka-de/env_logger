use std::io::{self, Write};

use super::Formatter;
use log::kv::{source::Source, Error, Key, Value, Visitor};

/// Format function for serializing key/value pairs
///
/// This function determines how key/value pairs for structured logs are serialized within the default
/// format.
pub(crate) type KvFormatFn = dyn Fn(&mut Formatter, &dyn Source) -> io::Result<()> + Sync + Send;

/// Null Key Value Format
///
/// This function is intended to be passed to
/// [`Builder::format_key_values`](crate::Builder::format_key_values).
///
/// This key value format simply ignores any key/value fields and doesn't include them in the
/// output.
pub fn hidden_kv_format(_formatter: &mut Formatter, _fields: &dyn Source) -> io::Result<()> {
    Ok(())
}

/// Default Key Value Format
///
/// This function is intended to be passed to
/// [`Builder::format_key_values`](crate::Builder::format_key_values).
///
/// This is the default key/value format. Which uses an "=" as the separator between the key and
/// value and a " " between each pair.
///
/// For example: `ip=127.0.0.1 port=123456 path=/example`
pub fn default_kv_format(formatter: &mut Formatter, fields: &dyn Source) -> io::Result<()> {
    fields
        .visit(&mut DefaultVisitor(formatter))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

struct DefaultVisitor<'a>(&'a mut Formatter);

impl<'a, 'kvs> Visitor<'kvs> for DefaultVisitor<'a> {
    fn visit_pair(&mut self, key: Key, value: Value<'kvs>) -> Result<(), Error> {
        // TODO: add styling
        // tracing-subscriber uses italic for the key and dimmed for the =
        write!(self.0, " {}={}", key, value)?;
        Ok(())
    }
}
