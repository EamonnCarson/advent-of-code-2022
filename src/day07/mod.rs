use std::{path::Path, rc::{Weak, Rc}, ops::Deref, collections::HashMap};

use crate::utils::{self, AOCResult, AOCError};

struct FileSystem {
    working_directory: Rc<FileSystemObject>,
    root_directory: Rc<FileSystemObject>,
}

impl FileSystem {
    fn cd<S: AsRef<str>>(&mut self, arg: S) {
        match arg.as_ref() {
            ".." => {
                self.working_directory = self.working_directory
                    .get_parent()
                    .expect("working directory has a parent since it exists");
            },
            "/" => {
                self.working_directory = self.root_directory.clone();
            }
            dir => {
                self.working_directory = self.working_directory.get_child(dir).unwrap();
            }
        };
    }
}

enum FileSystemObject {
    Directory {contents: HashMap<String, Rc<FileSystemObject>>, parent: Weak<FileSystemObject>},
    File {size: usize, parent: Weak<FileSystemObject>},
}

impl FileSystemObject {

    fn new_dir(parent: &Rc<FileSystemObject>) -> FileSystemObject {
        FileSystemObject::Directory { 
            contents: HashMap::new(), 
            parent: Rc::<FileSystemObject>::downgrade(parent) 
        }
    }

    fn get_parent(&self) -> Option<Rc<FileSystemObject>> {
        let parent = match self {
            FileSystemObject::Directory { parent, ..} => parent,
            FileSystemObject::File { parent, .. } => parent,
        };
        match parent.deref().upgrade() {
            Some(parent) => Some(parent),
            None => None,
        }
    }

    fn add_child(&mut self, object: FileSystemObject, name: String) -> AOCResult<()> {
        match self {
            FileSystemObject::Directory { contents, .. } => {
                if !contents.contains_key(&name) {
                    contents.insert(name, Rc::new(object));
                }
                return Ok(())
            }
            FileSystemObject::File {..} => Err(AOCError::new("file can't have children")),
        }
    }

    fn get_child<S: AsRef<str>>(&self, name: S) -> AOCResult<Rc<FileSystemObject>> {
        match self {
            FileSystemObject::Directory { contents, .. } => {
                let object = contents.get(name.as_ref());
                match object {
                    Some(object) => match object.deref() {
                        FileSystemObject::Directory {..} => Ok(object.clone()),
                        FileSystemObject::File {..} => Err(AOCError::new("Can't cd into a file")),
                    }
                    None => Err(AOCError::new(format!("no such directory {}", name.as_ref()))),
                }
            },
            FileSystemObject::File {..} => Err(AOCError::new("file has no subdirectories")),
        }
    }

    fn size(&self) -> usize {
        match self {
            FileSystemObject::Directory { contents, .. } => contents
                .values()
                .map(|obj| obj.size())
                .sum(),
            FileSystemObject::File { size, .. } => *size,
        }
    }

}

fn apply_command(line: String, file_system: &mut FileSystem) {
    let mut parts = line.split(" ");
    let size_or_mark = parts.next().unwrap();
    match size_or_mark {
        "$" => {
            let command = parts.next().unwrap();
            if command == "cd" {
                let arg = parts.next().unwrap();
                file_system.cd(arg);
            }
            // ignore ls, its useless
        },
        "dir" => {
            let name = parts.next().unwrap();
            file_system.working_directory.add_child(
                FileSystemObject::new_dir(&file_system.working_directory),
                name.to_owned()
            );
        }
        _ => {

        }
    }
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}