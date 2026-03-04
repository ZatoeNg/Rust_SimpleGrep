# Rust_SimpleGrep
这个是rust编写的简易版Grep项目。

# while let Ok(byte_read) = reader.read_line(&mut line)
* while let Ok(..)=....,只有reader成功,才会进入while
* let Ok(....) 表示模式匹配写法

```` rust
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    
    //写法1--错误处理
    let config = Config::new(&args)?; // 一行搞定错误处理

    /*写法2--错误处理
    let config_result = Config::new(&args);
    let config = match config_result {
        Ok(c) => c,
        Err(e) => {
        // 失败：返回错误，终止函数
        return Err(e);
        }
    };
    */
    for file in &config.files {
        grep_file(file, &config);
    }
    
    Ok(())
}
````
