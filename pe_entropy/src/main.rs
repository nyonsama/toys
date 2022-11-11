use memmap2::MmapOptions;
use pe::PE;
use std::fs::File;
use util::{calc_entropy, strip_0};

mod pe;
mod util;

fn calc_file_entropy(filename: &str) -> Result<f64, String> {
    // 打开文件
    let file =
        File::open(&filename).map_err(|err| format!("Open file error: {}", err.to_string()))?;

    // 使用mmap将文件映射到内存中
    let mmap = unsafe {
        MmapOptions::new()
            .map(&file)
            .map_err(|err| format!("Mmap error: {}", err.to_string()))?
    };

    // 构造用于分析PE文件的结构体
    let pe = PE::from_buf(&mmap);

    // 构造一个遍历节表的迭代器
    // 忽略.rsrc节、长度为0的节、只包含0x00的节，返回每个节的原始数据（去掉了末尾的0x00）
    let stripped_sections = pe.sections().filter_map(|s| {
        if s.header.name() == b".rsrc" {
            None
        } else {
            // 去掉这个节末尾的0x00
            let stripped = strip_0(s.data);
            if stripped.len() == 0 {
                None
            } else {
                Some(stripped)
            }
        }
    });

    // 计算每个节的信息熵并加权
    let mut entropy = 0f64;
    let mut size_sum = 0usize;
    for buf in stripped_sections {
        entropy += calc_entropy(buf) * buf.len() as f64;
        size_sum += buf.len();
    }
    Ok(entropy / size_sum as f64)
}
fn main() {
    // 解析命令行参数
    let mut args = std::env::args();
    let program_name = args.next().unwrap();

    // 获得第一个命令行参数
    if let Some(filename) = args.next() {
        // 计算信息熵
        match calc_file_entropy(&filename) {
            // 计算完成
            Ok(entropy) => {
                println!("{filename}: {entropy}");
            }
            // 错误处理
            Err(err) => {
                eprintln!("Calculate entropy failed: {}", err.to_string());
                eprintln!("Usage: {} filename", program_name);
            }
        }
    } else {
        // 没有参数，输出用法
        println!("Usage: {} filename", program_name);
    }
}
