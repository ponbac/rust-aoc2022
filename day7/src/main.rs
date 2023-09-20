use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

static EXAMPLE_INPUT: &str = r#"
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
"#;

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    // let input = EXAMPLE_INPUT;

    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree = Tree::new();
    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // ignore for now
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore for now
                    }
                    ".." => {
                        curr = tree.get(&curr)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_path) => {
                    // ignore for now
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry { path, size });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{}", s);

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node).unwrap())
        .filter(|&size| size <= 100_000)
        .sum::<u64>();

    println!("Part 1: {}", sum);

    let total_space = 70_000_000_u64;
    let space_needed = 30_000_000_u64;

    let space_used = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|node| node.children().is_empty())
        .map(|node| node.data().size)
        .sum::<u64>();
    let space_unused = total_space - space_used;
    let min_space_to_free = space_needed - space_unused;

    let dir_to_delete = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|node| !node.children().is_empty())
        .map(|node| (node.data().path.clone(), total_size(&tree, node).unwrap()))
        .filter(|(_, size)| size >= &min_space_to_free)
        .min_by_key(|(_, size)| *size)
        .unwrap();

    println!("Part 2: {:?}", dir_to_delete);

    Ok(())
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}
