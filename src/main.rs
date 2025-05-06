use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command;

fn main() {
    // 获取用户输入域名
    let domain = get_domain_input();

    // 读取并生成查询URL列表
    let urls = generate_urls(&domain);

    // 调用浏览器打开所有标签页
    open_firefox_with_urls(&urls);
}

fn get_domain_input() -> String {
    print!("请输入要查询的域名：");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string()
}

fn generate_urls(domain: &str) -> Vec<String> {
    let mut urls = Vec::new();

    // 读取包含URL模板的文件
    match File::open("ti_sites.txt") {
        Ok(file) => {
            for line in io::BufReader::new(file).lines() {
                if let Ok(url_template) = line {
                    let url = url_template.replace("{domain}", domain);
                    urls.push(url);
                }
            }
        }
        Err(e) => eprintln!("无法打开文件: {}", e),
    }

    urls
}

fn open_firefox_with_urls(urls: &[String]) {
    if urls.is_empty() {
        println!("未找到有效的URL模板");
        return;
    }

    // 构建浏览器命令
    let mut cmd = Command::new("firefox");
    cmd.arg("--new-window");

    // 添加所有URL参数
    for url in urls {
        cmd.arg(url);
    }

    // 执行命令
    match cmd.spawn() {
        Ok(_) => println!("已在Firefox中打开{}个查询页面", urls.len()),
        Err(e) => eprintln!("执行失败: {}\n请确保Firefox已安装并在PATH中", e),
    }
}