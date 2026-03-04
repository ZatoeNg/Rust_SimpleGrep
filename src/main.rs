// 1. 导入 std::fs::File 模块：用于操作文件（打开、读取等）
use std::fs::File;
// 2. 导入 std::io 中的 BufRead 和 BufReader：
//    - BufRead：提供逐行读取的方法（read_line）
//    - BufReader：带缓冲区的读取器，比直接读文件快很多
use std::io::{BufRead, BufReader};

// 3. 定义结构体 Config：用来存储程序的核心配置（所有配置项集中管理）
struct Config {
    // 4. 要搜索的关键词（字符串类型，所有权属于 Config）
    pattern: String,
    // 5. 要搜索的文件列表（Vec<String> 是字符串数组）
    files: Vec<String>,
    // 6. 是否忽略大小写（布尔值：true/false）
    case_insensitive: bool,
}

// 7. 为 Config 实现方法（impl 是 implementation 的缩写）
impl Config {
    // 8. 定义 new 函数：接收命令行参数数组，返回 Result 类型
    //    - 成功：返回 Config 实例（Ok(Config)）
    //    - 失败：返回错误字符串（Err(String)）
    //    - &[String]：参数是字符串切片（借用，不拿所有权）
    fn new(args: &[String]) -> Result<Config, String> {
        // 9. 检查参数数量：args[0] 是程序名，至少需要 3 个参数（程序名+关键词+文件名）
        if args.len() < 3 {
            // 10. 参数不够，返回错误：format! 拼接字符串，提示正确用法
            return Err(format!(
                "Usage: {} <pattern> <files>(or file [-i])",
                args[0]
            ));
        }

        // 11. 初始化忽略大小写为 false（默认区分大小写）
        let mut case_insensitive = false;
        // 12. 初始化空数组：存储非标志参数（关键词、文件名，排除 -i）
        let mut non_flag_args: Vec<String> = Vec::new();

        // 13. 遍历命令行参数（跳过 args[0] 程序名）
        //    - &args[1..]：取 args 从第 1 个元素到末尾的切片（借用，不复制）
        for arg in &args[1..] {
            // 14. 如果参数是 -i，开启忽略大小写
            if arg == "-i" {
                case_insensitive = true;
            } else {
                // 15. 非 -i 参数：克隆（复制）后存入 non_flag_args
                //    - clone()：因为 arg 是借用，需要拿到所有权才能存到数组里
                non_flag_args.push(arg.clone());
            }
        }

        // 16. 检查是否有关键词：non_flag_args 为空 → 没传关键词
        if non_flag_args.is_empty() {
            return Err("Error:Pattern not provided".to_string());
        }

        // 17. 取第一个非标志参数作为搜索关键词（克隆拿到所有权）
        let pattern = non_flag_args[0].clone();

        // 18. 检查是否有文件名：非标志参数至少需要 2 个（关键词+文件名）
        if non_flag_args.len() < 2 {
            return Err("Error:No input files provided".to_string());
        }

        // 19. 取非标志参数从第 1 个到末尾的所有元素，作为文件列表
        let files: Vec<String> = non_flag_args[1..].to_vec();

        // 20. 所有校验通过，返回 Config 实例（Ok 包裹）
        Ok(Config {
            pattern,          // 等同于 pattern: pattern
            files,            // 等同于 files: files
            case_insensitive, // 等同于 case_insensitive: case_insensitive
        })
    }
}

// 21. 定义 grep 函数：接收缓冲读取器、关键词、是否忽略大小写
//    - &mut BufReader<File>：可变引用（因为 read_line 会修改读取器的状态）
fn grep(reader: &mut BufReader<File>, pattern: &str, case_insensitive: bool) {
    // 22. 初始化空字符串：存储每次读取的文件行（复用这个字符串，减少内存分配）
    let mut line: String = String::new();

    // 23. 循环读取每一行：read_line 尝试读取一行到 line 里，返回读取的字节数
    //    - while let Ok(byte_read)：只处理“读取成功”的情况，失败会直接退出循环
    while let Ok(byte_read) = reader.read_line(&mut line) {
        // 24. 字节数为 0 → 读到文件末尾，退出循环
        if byte_read == 0 {
            break;
        }

        // 25. 声明 matched 变量：存储“当前行是否匹配”的结果
        let matched;
        // 26. 判断是否忽略大小写
        if case_insensitive {
            // 27. 都转小写后匹配：to_lowercase() 返回新字符串，contains 判断是否包含
            matched = line.to_lowercase().contains(&pattern.to_lowercase());
        } else {
            // 28. 不忽略大小写：直接匹配原始字符串
            matched = line.contains(pattern);
        }

        // 29. 如果匹配成功，打印这一行
        if matched {
            println!("{}", line);
        }

        // 30. 清空 line：必须做！否则下一次 read_line 会把新行拼到旧行后面
        line.clear();
    }
}

// 31. 定义 grep_file 函数：接收文件名、配置（都是借用，不拿所有权）
fn grep_file(file: &str, config: &Config) {
    // 32. 尝试打开文件：File::open 返回 Result<File, io::Error>
    match File::open(file) {
        // 33. 文件打开成功：创建缓冲读取器
        Ok(f) => {
            let mut reader = BufReader::new(f);
            // 34. 调用 grep 函数：传入可变的 reader（因为 grep 需要 &mut）
            grep(&mut reader, &config.pattern, config.case_insensitive);
        }
        // 35. 文件打开失败：打印错误信息（eprintln 输出到错误流，不混在正常输出里）
        Err(e) => {
            eprintln!("Could not open file: {}:{}", file, e);
        }
    }
}

// 36. main 函数：程序的入口，返回 Result<(), String>（允许返回错误）
fn main() -> Result<(), String> {
    // 37. 收集命令行参数：std::env::args() 返回参数迭代器，collect 转为 Vec<String>
    let args: Vec<String> = std::env::args().collect();
    // 38. 解析参数生成配置：? 运算符——如果 Err 直接返回错误，终止程序；如果 Ok 取出 Config
    let config = Config::new(&args)?;

    // 39. 遍历所有文件：&config.files 是借用，避免移动所有权
    for file in &config.files {
        // 40. 处理每个文件：调用 grep_file
        grep_file(file, &config);
    }

    // 41. 程序正常结束，返回 Ok(())
    Ok(())
}
