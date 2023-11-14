use std::collections::VecDeque;

/*
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
*/

#[derive(Debug)]
enum Type {
    Dir(String),         // name
    File(usize, String), // size, name
}

#[derive(Debug)]
enum Command {
    CdIn(String),  // cd a
    CdOut,         // cd ..
    Ls(Vec<Type>), // ls
}

#[derive(Debug)]
struct DirNode {
    name: String,
    subdirs: Vec<DirNode>,
    files: Vec<(usize, String)>,
}

fn parse_ls_entry(line: &str) -> Type {
    let mut parts = line.split_whitespace();
    let first = parts.next().unwrap();
    if first == "dir" {
        let name = parts.next().unwrap().to_string();
        Type::Dir(name)
    } else {
        let size = first.parse::<usize>().unwrap();
        let name = parts.next().unwrap().to_string();
        Type::File(size, name)
    }
}

fn parse_input(contents: String) -> DirNode {
    let mut lines = contents.lines();

    let mut root = DirNode {
        name: "/".to_string(),
        subdirs: Vec::new(),
        files: Vec::new(),
    };

    todo!()
}

#[test]
fn main() {
    use super::*;

    let contents = read_input("d7.txt");
}
