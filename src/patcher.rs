use crate::edit::Edit;
use crate::replacer::Replacer;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str;

pub(crate) struct Patcher<'a> {
    edits: Vec<Edit>,
    replacer: Option<&'a Replacer>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid line number {0}")]
    LineNumber(u32),
}

impl<'a> Patcher<'a> {
    pub(crate) fn new(edits: Vec<Edit>, replacer: Option<&'a Replacer>) -> Self {
        Self { edits, replacer }
    }

    pub(crate) fn patch(&self, mut lines: Vec<String>, delete: bool) -> Result<String, Error> {
        if delete {
            let mut indexes: Vec<u32> = self.edits.iter().map(|e| e.line_number).collect();
            indexes.sort_unstable();
            indexes.dedup();
            indexes.reverse();

            // Subtract `1` from the line number because line numbers start from `1` and array
            // indices start from `0`
            for index in indexes {
                let index_cloned = index.clone();
                let index_size = usize::try_from(index_cloned).unwrap() - 1;
                // sanity check if line number is legit
                if index_size >= lines.len().try_into().unwrap() {
                    return Err(Error::LineNumber(index_cloned));
                }
                lines.remove(index_size);
            }
            return Ok(lines.join("\n"));
        }
        for edit in &self.edits {
            // Subtract `1` from the line number because line numbers start from `1` but array
            // indices start from `0`
            let index = usize::try_from(edit.line_number).unwrap() - 1;
            if index >= lines.len().try_into().unwrap() {
                return Err(Error::LineNumber(edit.line_number));
            }
            if let Some(replacer) = &self.replacer {
                let replaced = &replacer.replace(edit.text.as_bytes());
                // Convert cow to &str and check if conversion is some.
                let result = str::from_utf8(replaced);
                let text = match result {
                    Ok(result) => result,
                    Err(err) => panic!("Error replacing: {}", err), // FIXME:
                };
                // The early for-loop is to remove the target line (without modify) and
                // this for-loop replace removed line with a modified text.
                // TODO: We can improve this.
                lines[index] = text.to_string();
            } else {
                // if replacer is none, just copy back
                lines[index] = edit.text.clone();
            }
        }
        return Ok(lines.join("\n"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn patch_bad_number() {
        let patcher = Patcher::new(
            vec![
                Edit {
                    file: PathBuf::from("f"),
                    line_number: 1,
                    text: "foo".to_string(),
                },
                Edit {
                    file: PathBuf::from("f"),
                    line_number: 3,
                    text: "bar".to_string(),
                },
            ],
            None,
        );
        let lines = vec!["a".to_string(), "b".to_string()];
        let result = patcher.patch(lines, false);
        assert!(matches!(result, Err(Error::LineNumber(3))));
    }

    #[test]
    fn patch() {
        let patcher = Patcher::new(
            vec![
                Edit {
                    file: PathBuf::from("f"),
                    line_number: 2,
                    text: "foo".to_string(),
                },
                Edit {
                    file: PathBuf::from("f"),
                    line_number: 3,
                    text: "bar".to_string(),
                },
            ],
            None,
        );
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = patcher.patch(lines, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a\nfoo\nbar");
    }
}
