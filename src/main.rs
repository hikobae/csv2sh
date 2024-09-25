use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Hash)]
struct User {
    name: String,
    mail: String,
}

impl User {
    fn new(name: &str, mail: &str) -> Self {
        User {
            name: name.to_string(),
            mail: mail.to_string(),
        }
    }
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Usage: csv2sh CSV_FILE");
        std::process::exit(1);
    }

    let csv_path = env::args().nth(1).unwrap();

    if let Err(err) = csv_file2sh(&csv_path) {
        eprintln!("{:#?}", err);
        std::process::exit(1);
    }
}

fn new_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        String::from("taro"),
        User::new("Test Test", "test.test@example.com"),
    );
    users
}

fn csv_file2sh(csv_path: &str) -> Result<()> {
    let csv = fs::read_to_string(csv_path)?;
    println!("{}", csv2sh(&csv)?);
    Ok({})
}

fn csv2sh(csv: &str) -> Result<String> {
    let output_dir = "out";
    if Path::new(output_dir).exists() {
        return Err(anyhow!("Directory {output_dir} already exists"));
    }

    let users = new_users();

    let mut sh: String = String::new();
    sh += &format!("mkdir {output_dir}\n");
    sh += &format!("pushd {output_dir}\n");
    sh += "git init\n";

    type Record = (
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    );
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        match record.0.to_lowercase().as_str() {
            "commit" => {
                let arg1 = record.1.unwrap();
                println!("{}", arg1);
                let date = NaiveDateTime::parse_from_str(&arg1, "%Y/%m/%d %H:%M:%S")
                    .unwrap()
                    .format("%Y-%m-%dT%H:%M:%S")
                    .to_string();

                let user_id = record.2.unwrap();
                let user = users.get(&user_id).unwrap();

                let mut input = record.3.unwrap();
                input += "/.";
                let comment = record.4.unwrap();
                sh += "find . -not -path './.git*' -not -path '.' -print0 | xargs -0 rm -rf\n";
                sh += &format!("cp -r {input} .\n");
                sh += "git add -A\n";
                sh += &format!("GIT_COMMITTER_NAME=\"{name}\" GIT_COMMITTER_EMAIL=\"{mail}\" GIT_COMMITTER_DATE=\"{date}\" GIT_AUTHOR_NAME=\"{name}\" GIT_AUTHOR_EMAIL=\"{mail}\" GIT_AUTHOR_DATE=\"{date}\" git commit -m {comment}\n", name=user.name, mail=user.mail, date=date);
            }
            "tag" => sh += &format!("git tag {}\n", record.4.unwrap()),
            _ => return Err(anyhow!("Unknown command `{}`", record.0)),
        }
    }
    sh += "popd\n";
    Ok(sh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv2sh_test() {
        let input = r#"COMMAND,ARG1,ARG2,ARG3,ARG4
commit,2014/02/03 03:40:00,taro,../input/1,test
commit,2014/12/03 15:32:00,taro,../input/2,test2
tag,,,,docs/ios/1.0.0
"#;
        let expect = r#"mkdir out
pushd out
git init
find . -not -path './.git*' -not -path '.' -print0 | xargs -0 rm -rf
cp -r ../input/1/. .
git add -A
GIT_COMMITTER_NAME="Test Test" GIT_COMMITTER_EMAIL="test.test@example.com" GIT_COMMITTER_DATE="2014-02-03T03:40:00" GIT_AUTHOR_NAME="Test Test" GIT_AUTHOR_EMAIL="test.test@example.com" GIT_AUTHOR_DATE="2014-02-03T03:40:00" git commit -m test
find . -not -path './.git*' -not -path '.' -print0 | xargs -0 rm -rf
cp -r ../input/2/. .
git add -A
GIT_COMMITTER_NAME="Test Test" GIT_COMMITTER_EMAIL="test.test@example.com" GIT_COMMITTER_DATE="2014-12-03T15:32:00" GIT_AUTHOR_NAME="Test Test" GIT_AUTHOR_EMAIL="test.test@example.com" GIT_AUTHOR_DATE="2014-12-03T15:32:00" git commit -m test2
git tag docs/ios/1.0.0
popd
"#;
        let result = csv2sh(input).unwrap();
        assert_eq!(result, expect);
    }
}
