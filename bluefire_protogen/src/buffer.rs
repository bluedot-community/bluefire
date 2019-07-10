// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains definition of the `Buffer`.

/// Helper structure for building indented code. Keeps string contents and current indent value.
pub struct Buffer {
    content: Vec<String>,
    current: String,
    indent: usize,
}

impl Buffer {
    /// Creates a new `Buffer`.
    pub fn new(indent: usize) -> Self {
        Buffer { content: Vec::new(), current: String::new(), indent: indent }
    }

    /// Returns the current buffer content as a `String`.
    pub fn get_content(&mut self) -> String {
        self.flush();
        let mut size = 0;
        for iter in self.content.iter() {
            size += iter.len() + 1;
        }
        let mut content = String::with_capacity(size);
        for iter in self.content.iter() {
            content += iter;
            content += "\n";
        }
        content
    }

    /// Creates a new line.
    pub fn flush(&mut self) -> &mut Buffer {
        self.content.push(self.current.clone());
        self.current = String::new();
        self
    }

    /// Pushes current indent.
    pub fn push_indent(&mut self) -> &mut Buffer {
        for _ in 0..self.indent {
            self.current.push_str("    ");
        }
        self
    }

    /// Increases level in indentation.
    pub fn increase_indent(&mut self) -> &mut Buffer {
        self.indent += 1;
        self
    }

    /// Decreases level in indentation.
    pub fn decrease_indent(&mut self) -> &mut Buffer {
        self.indent -= 1;
        self
    }

    /// Creates a new line.
    pub fn new_line(&mut self) -> &mut Buffer {
        self.flush();
        self
    }

    /// Adds new content without adding an indent nor creating a new line.
    pub fn push(&mut self, content: &str) -> &mut Buffer {
        self.current += content;
        self
    }

    /// Adds new content with indent and creates a new line.
    pub fn push_line(&mut self, content: &str) -> &mut Buffer {
        self.push_indent();
        self.push(content);
        self.flush();
        self
    }

    /// Pushes a multi-line content.
    ///
    /// The first line must be empty. All the lines are expected to have the indentation at least
    /// as wide as the second line or to be empty. This indentation will be replaced with the
    /// buffers current indentation.
    pub fn push_multiline(&mut self, content: &String) -> &mut Buffer {
        fn count_leading_spaces(line: &str) -> usize {
            let mut count = 0;
            for character in line.chars() {
                if character == ' ' {
                    count += 1;
                } else {
                    break;
                }
            }
            count
        }

        fn calculate_indent(line: &str) -> usize {
            let spaces = count_leading_spaces(line);
            if (spaces % 4) == 0 {
                spaces / 4
            } else {
                panic!("Line '{}' has incorrect indent", line)
            }
        }

        fn trim_for_indent(line: &str, indent: usize) -> &str {
            let indent = std::cmp::min(indent, line.len());
            let (first, last) = line.split_at(indent);
            let spaces = count_leading_spaces(first);
            if spaces != indent {
                panic!("Line '{}' has incorrect indent", line)
            }
            last
        }

        let mut indent = 0;
        for (num, line) in content.lines().enumerate() {
            if num == 0 {
                if line != "" {
                    panic!("In multi-line mode the first line must be empty");
                }
            } else {
                if num == 1 {
                    indent = calculate_indent(&line);
                }
                let contents = trim_for_indent(&line, 4 * indent);
                if contents.len() != count_leading_spaces(contents) {
                    self.push_line(trim_for_indent(&line, 4 * indent));
                } else {
                    self.new_line();
                }
            }
        }
        self
    }
}
