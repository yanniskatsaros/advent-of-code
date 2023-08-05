// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_while},
//     character::complete::{char as pchar, multispace0, u64 as pu64},
//     combinator::{map, rest, value},
//     sequence::{delimited, preceded, separated_pair, tuple},
//     IResult,
// };
//
// #[derive(Debug, Eq, PartialEq)]
// enum FsCmd {
//     Ls,
//     Cd(String),
// }
//
// #[derive(Debug, Eq, PartialEq)]
// enum FsItem {
//     Dir(String),
//     File(u64, String),
// }
//
// #[derive(Debug, Eq, PartialEq)]
// enum FsLine {
//     Cmd(FsCmd),
//     Item(FsItem),
// }
//
// fn parse_command(s: &str) -> IResult<&str, FsCmd> {
//     let ls = map(tag("ls"), |_| FsCmd::Ls);
//     let cd = map(
//         separated_pair(tag("cd"), multispace0, rest),
//         |(_, r): (&str, &str)| FsCmd::Cd(r.to_string()),
//     );
//     let cmd = alt((ls, cd));
//
//     let (rest, (_, result)) = separated_pair(pchar('$'), multispace0, cmd)(s)?;
//
//     Ok((rest, result))
// }
//
// fn parse_item(s: &str) -> IResult<&str, FsItem> {
//     let dir = map(
//         separated_pair(tag("dir"), multispace0, rest),
//         |(_, name): (&str, &str)| FsItem::Dir(name.to_string()),
//     );
//     let file = map(
//         separated_pair(pu64, multispace0, rest),
//         |(size, name): (u64, &str)| FsItem::File(size, name.to_string()),
//     );
//
//     alt((dir, file))(s)
// }
//
// fn parse_line(s: &str) -> IResult<&str, FsLine> {}
//
fn main() {}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_cmd_ls() {
//         assert_eq!(parse_command("$ ls"), Ok(("", FsCmd::Ls)));
//     }
//
//     #[test]
//     fn parse_cmd_cd() {
//         assert_eq!(
//             parse_command("$ cd /"),
//             Ok(("", FsCmd::Cd("/".to_string())))
//         );
//     }
//
//     #[test]
//     fn parse_item_dir() {
//         assert_eq!(parse_item("dir a"), Ok(("", FsItem::Dir("a".to_string()))));
//     }
//
//     #[test]
//     fn parse_item_file() {
//         assert_eq!(
//             parse_item("14848514 b.txt"),
//             Ok(("", FsItem::File(14848514, "b.txt".to_string())))
//         );
//     }
// }
