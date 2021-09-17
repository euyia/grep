/* 其实就做了三件事。
1 就是collect 输入
2 转成 数据结构实例
3 用他的里面的元素作为参数。用fs 去查文件内容。
 Box<dyn Error>> 这个意思 就是返回的是一个实现了error trait的类型
 为什么要这样去区分呢。。因为。。这样说明你放回的那个东西 是能够用error
 trait 里面的那些函数 这就够了。*/
use std::env;
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    // 它不是vec类型吗 为什么传进去就编程数组切片了

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        // 这就是对Result 如果的错误的情况进行考虑到。
        // 这个错误变体..存在 .那么做下面的处理.
        println!("{}", e);
        process::exit(1);
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("{}", contents);
    Ok(())
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        // &args[1]它是先拿到数组所对应的一个index的值。。然后再做&取地址
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };
        Ok(Config { query, filename })
        //  外面包一层 是Result的需求。这样才能对应放回值里面的设定把
        // 和Err 返回。。刚好对应返回值的两种情况
        // 不求甚解。只要知道是什么意思就行了。表达了什么就行了
        // 要在能做的这个维度上 做努力 而不是去解释。
    }
}
