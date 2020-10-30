# csv_sjis

Rustで、SHIFT_JISのCSVを読むテスト。

ほぼ
[character encoding - How to read a non-UTF8 encoded csv file? - Stack Overflow](https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file)
からのコピペ。

`cargo run`
すると
`./test/csv_sjis/sjis.csv`
から読んで

```
$ cargo run
   Compiling csv_sjis v0.1.0 (/home/heiwa/rust/csv_sjis)
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/csv_sjis`
StringRecord(["1", "りんご", "apple"])
StringRecord(["2", "バナナ", "banana"])
StringRecord(["3", "さくさんぼ", "cherry"])
```
のような出力が得られる。


# 参考

- [encoding_rs - Rust](https://docs.rs/encoding_rs/0.8.24/encoding_rs/)
- [encoding_rs_io - Rust](https://docs.rs/encoding_rs_io/0.1.4/encoding_rs_io/index.html)
- [encoding_rs_io::DecodeReaderBytesBuilder - Rust](https://docs.rs/encoding_rs_io/0.1.4/encoding_rs_io/struct.DecodeReaderBytesBuilder.html)
- [csv - Rust](https://docs.rs/csv/1.1.3/csv/)
- [csv::StringRecord - Rust](https://docs.rs/csv/1.1.3/csv/struct.StringRecord.html)
- [character encoding - How to read a non-UTF8 encoded csv file? - Stack Overflow](https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file)
- [BurntSushi/ripgrep: ripgrep recursively searches directories for a regex pattern while respecting your gitignore](https://github.com/BurntSushi/ripgrep)
