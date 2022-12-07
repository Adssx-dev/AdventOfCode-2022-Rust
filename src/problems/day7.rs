
enum EntryType {
    File{size : u32},
    Folder{children : Vec<usize>}
}

struct Entry {
    entry_type : EntryType,
    name : String,
    parent : Option<usize>
}

impl Entry {
    pub fn from_str(line : &str, parent : Option<usize>) -> Entry{
        match line.chars().next() {
            Some('d') => Entry {
                name : line.chars().into_iter().skip(4).collect(),
                entry_type : EntryType::Folder { children: vec!() },
                parent
            },
            Some(_) => {
                let mut splitted = line.split(' ');
                Entry {
                    entry_type : EntryType::File { size: splitted.next().unwrap().parse::<u32>().unwrap() },
                    name : splitted.next().unwrap().to_owned(),
                    parent
                }
            }
            None => panic!("Unexpected empty line")
        }
    }
}

struct EntryList {
    entries : Vec<Entry> ,
    current_entry : Option<usize>
}

impl EntryList {
    pub fn process_line(&mut self, line : &str) {
        match line.chars().next() {
            Some('$') => self.process_command(&line[2..]),
            Some(_) => self.process_entry(line),
            None => panic!("Unexpected empty line")
        }
    }

    fn process_entry(&mut self, entry_str : &str) {
        let new_entry = Entry::from_str(entry_str, self.current_entry);
        self.entries.push(new_entry);
        let last_index = self.entries.len() - 1;
        if self.current_entry != None {
            if let EntryType::Folder { ref mut children } = &mut self.entries[self.current_entry.unwrap()].entry_type {
                children.push(last_index);
            }
        }
    }

    fn process_command(&mut self, line:&str) {
        match line.chars().next(){
            Some('c') => self.cd_command(&line[3..]),
            Some('l') => {}, // Nothing to do with this command
            Some(_) => panic!("Unexpected command"),
            None => panic!("No command")
        }
    }

    fn cd_command(&mut self, path : &str) {
        self.current_entry = match path {
            ".." => self.entries[self.current_entry.unwrap()].parent,
            file_name => Some(self.find_folder_index_in_children(file_name))
        }
    }

    fn find_folder_index_in_children(&self, name : &str) -> usize {
        if self.current_entry == None {
            return 0;
        }
        else if let EntryType::Folder { children } = &self.entries[self.current_entry.unwrap()].entry_type {
            for c in children {
                let child = &self.entries[*c];
                if child.name == name {
                    return *c;
                }
            }
        }
        panic!("Could not find child");
    }

    pub fn get_size(& self, id : usize) -> u32 {
        match &self.entries[id].entry_type {
            EntryType::File { size } => *size,
            EntryType::Folder { children } => children.iter().map(|c| self.get_size(*c)).sum()
        }
    }

    pub fn is_folder(&self, id : usize) -> bool {
        matches!(&self.entries[id].entry_type, EntryType::Folder{children:_})
    }

}

pub fn day7_pt1() -> u32 {
    let file = include_str!("../../inputs/day7.txt");

    let mut entry_list = EntryList {
        entries : vec!(),
        current_entry : None
    };

    entry_list.process_line("dir /");

    for l in file.lines() {
        entry_list.process_line(l);
    }

    (1..entry_list.entries.len())
        .filter(|id| entry_list.is_folder(*id))
        .map(|id| entry_list.get_size(id))
        .filter(|size| *size < 100000)
        .sum()
       
}


pub fn day7_pt2() -> u32 {
    let file = include_str!("../../inputs/day7.txt");

    let mut entry_list = EntryList {
        entries : vec!(),
        current_entry : None
    };

    entry_list.process_line("dir /");

    for l in file.lines() {
        entry_list.process_line(l);
    }

    let available_space = 70000000 - entry_list.get_size(0);
    let needed_space = 30000000 - available_space;

    (1..entry_list.entries.len())
        .filter(|id| entry_list.is_folder(*id))
        .map(|id| entry_list.get_size(id))
        .filter(|size| *size >= needed_space)
        .min()
        .unwrap()
       
}

#[cfg(test)]
mod tests {
    use crate::problems::day7::*;

    #[test]
    fn day7_pt1_test() {
        let result = day7_pt1();
        assert_eq!(result, 1770595);
    }

    #[test]
    fn day7_pt2_test() {
        let result = day7_pt2();
        assert_eq!(result, 2195372);
    }
}
