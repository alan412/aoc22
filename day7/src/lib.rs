use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum DirInfo {
    File(u32),
    Dir(Vec<Rc<RefCell<DirEntry>>>),
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    info: DirInfo,
}

impl DirEntry {
    pub fn new_dir(name: String) -> Self {
        Self {
            name,
            info: DirInfo::Dir(Vec::new()),
        }
    }
    pub fn new_file(name: String, size: u32) -> Self {
        Self {
            name,
            info: DirInfo::File(size),
        }
    }
    pub fn add_new_dir(&mut self, name: String) {
        match self.info {
            DirInfo::File(_) => panic!("Can't add to a file"),
            DirInfo::Dir(ref mut v) => v.push(Rc::new(RefCell::new(Self::new_dir(name)))),
        }
    }
    pub fn add_new_file(&mut self, name: String, size: u32) {
        match self.info {
            DirInfo::File(_) => panic!("Can't add to a file"),
            DirInfo::Dir(ref mut v) => v.push(Rc::new(RefCell::new(Self::new_file(name, size)))),
        }
    }
    pub fn get_size(&self) -> u32 {
        match self.info {
            DirInfo::File(size) => size,
            DirInfo::Dir(ref v) => {
                let mut total: u32 = 0;
                for item in v {
                    total += item.borrow().get_size();
                }
                total
            }
        }
    }

    pub fn get_subdir(&self, name: String) -> Rc<RefCell<DirEntry>> {
        match self.info {
            DirInfo::File(_) => panic!("Can't get a subdir from a file!"),
            DirInfo::Dir(ref v) => {
                for item in v {
                    let curr_item = item.borrow();
                    if curr_item.name == name {
                        return Rc::clone(item);
                    }
                }
                panic!("No subdir with the name: {}", name);
            }
        }
    }

    pub fn print_tree(&self, num_dashes: usize) {
        println!(
            "{}{} {}",
            "-".repeat(num_dashes),
            self.name,
            self.get_size(),
        );
        match self.info {
            DirInfo::File(_) => {}
            DirInfo::Dir(ref v) => {
                for item in v {
                    item.borrow().print_tree(num_dashes + 1);
                }
            }
        }
    }
    pub fn get_less_than(&self, size: u32) -> u32 {
        let mut total = 0;
        match self.info {
            DirInfo::File(_) => 0,
            DirInfo::Dir(ref v) => {
                for item in v {
                    total += item.borrow().get_less_than(size);
                }
                let dir_size = self.get_size();
                println!("{} {}", self.name, dir_size);
                if dir_size < size {
                    dir_size + total
                } else {
                    total
                }
            }
        }
    }
    pub fn get_smallest_dir_larger_than(&self, size: u32) -> u32 {
        match self.info {
            DirInfo::File(_) => u32::MAX,
            DirInfo::Dir(ref v) => {
                let mut smallest = u32::MAX;

                for item in v {
                    let result = item.borrow().get_smallest_dir_larger_than(size);
                    if result < smallest {
                        smallest = result
                    }
                    if smallest == u32::MAX {
                        // hasn't been found yet
                        let our_size = self.get_size();
                        if our_size > size {
                            smallest = our_size;
                        }
                    }
                }
                smallest
            }
        }
    }
}
