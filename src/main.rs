/// 代码核心逻辑由AI生成。
/// 提示词：我想用Rust编写一个程序，当前目录中的bundle文件夹中有一个java的jre，还有一个打好的JAR包，我希望你把bundle也捆绑进rust的编译结果（可执行文件.exe）中，程序的核心逻辑就是运行java -jar + jar包的路径，其中java命令在jre中。
/// 最后我想要的代码经过cargo build --release后在windows平台上是一个.exe文件，双击它打开可以使用exe中捆绑的jre运行这个jar包
use include_dir::{include_dir, Dir};
use std::{fs, path::Path, process::Command};
use tempfile::tempdir;

// 嵌入 bundle 目录
static BUNDLE: Dir = include_dir!("bundle/");

fn main() {
    // 创建临时目录
    let dir = tempdir().expect("无法创建临时目录");
    let temp_path = dir.path();

    // 解压 bundle 目录到临时目录
    extract_bundle(temp_path).expect("解压 bundle 失败");

    // 获取 JRE 和 JAR 文件路径
    let jre_path = temp_path.join("jre/bin/java.exe");
    if !jre_path.exists() {
        panic!("找不到 JRE 文件");
    }
    let jar_path = fs::read_dir(temp_path)
        .unwrap()
        .find(|entry| {
            entry
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .contains("jar")
        })
        .expect("找不到 JAR 文件")
        .expect("无法读取 JAR 文件")
        .path();

    // 执行 java -jar
    let output = Command::new(jre_path)
        .arg("-jar")
        .arg(jar_path)
        .spawn()
        .expect("启动 Java 进程失败");

    // 等待进程结束
    let _ = output.wait_with_output();
}

fn extract_bundle(dest: &Path) -> std::io::Result<()> {
    for entry in BUNDLE.files() {
        let path = dest.join(entry.path());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, entry.contents())?;
    }
    Ok(())
}
