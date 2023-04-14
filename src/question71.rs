use super::Solution;

// Question 71. Simplify Path

// Given a string path, which is an absolute path (starting with a slash '/') to a file or directory in a Unix-style file system, convert it to the simplified canonical path.

// In a Unix-style file system, a period '.' refers to the current directory, a double period '..' refers to the directory up a level, and any multiple consecutive slashes (i.e. '//') are treated as a single slash '/'. For this problem, any other format of periods such as '...' are treated as file/directory names.

// The canonical path should have the following format:

// The path starts with a single slash '/'.
// Any two directories are separated by a single slash '/'.
// The path does not end with a trailing '/'.
// The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period '.' or double period '..')
// Return the simplified canonical path.

#[allow(unused)]

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let keys = path.split("/").filter(|s| s != &"").collect::<Vec<&str>>();
        let mut vec = vec![];
        let mut output_string = String::new();

        for key in keys {
            if key == "." {
                continue;
            } else if key == ".." {
                vec.pop();
            } else {
                vec.push(key);
            }
        }

        let len = vec.len();

        if len == 0 {
            output_string.push_str("/");
        } else {
            for key in vec {
                output_string.push_str(format!("/{}", key).as_str());
            }
        }

        return output_string;
    }
}
