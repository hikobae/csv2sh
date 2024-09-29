# csv2sh

csv ファイルからシェルスクリプトを生成する.

## Usage

```sh
$ cat command.csv
COMMAND,ARG1,ARG2,ARG3,ARG4
commit,2014/02/03 03:40:00,taro,../input/1,test
commit,2014/12/03 15:32:00,taro,../input/2,test2
tag,,,,docs/ios/1.0.0
commit,2015/01/12 23:59:59,hanako,../input/3,Update files

$ cat users.csv
ID,NAME,EMAIL
taro,OSAKA Taro,taro@example.com
hanako,Tokyo Hanako,hanako@example.org

$ csv2sh command.csv users.csv
mkdir out
pushd out
git init
find . -not -path './.git*' -not -path '.' -print0 | xargs -0 rm -rf
cp -r ../input/1/. .
git add -A
GIT_COMMITTER_NAME="OSAKA Taro" GIT_COMMITTER_EMAIL="taro@example.com" GIT_COMMITTER_DATE="2014-02-03T03:40:00" GIT_AUTHOR_NAME="OSAKA Taro" GIT_AUTHOR_EMAIL="taro@example.com" GIT_AUTHOR_DATE="2014-02-03T03:40:00" git commit -m "test"
find . -not -path './.git*' -not -path '.' -print0 | xargs -0 rm -rf
cp -r ../input/2/. .
git add -A
GIT_COMMITTER_NAME="OSAKA Taro" GIT_COMMITTER_EMAIL="taro@example.com" GIT_COMMITTER_DATE="2014-12-03T15:32:00" GIT_AUTHOR_NAME="OSAKA Taro" GIT_AUTHOR_EMAIL="taro@example.com" GIT_AUTHOR_DATE="2014-12-03T15:32:00" git commit -m "test2"
git tag docs/ios/1.0.0
find . -not -path './.git*' -not -path '.' -print0 | xargs -0 rm -rf
cp -r ../input/3/. .
git add -A
GIT_COMMITTER_NAME="Tokyo Hanako" GIT_COMMITTER_EMAIL="hanako@example.org" GIT_COMMITTER_DATE="2015-01-12T23:59:59" GIT_AUTHOR_NAME="Tokyo Hanako" GIT_AUTHOR_EMAIL="hanako@example.org" GIT_AUTHOR_DATE="2015-01-12T23:59:59" git commit -m "Update files"
popd
```

## License

MIT License

## Author

TAKAHASHI Satoshi <hikobae@gmail.com>
