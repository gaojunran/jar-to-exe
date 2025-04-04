use include_dir::{include_dir, Dir};
use std::{
    fs::{self},
    path::Path,
    process::Command,
};

// 嵌入 bundle 目录
static BUNDLE: Dir = include_dir!("bundle");

fn main() {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let temp_path = home_dir.join("java_temp");
    fs::create_dir_all(&temp_path).expect("Failed to create temp directory");

    // 解压 bundle 目录到临时目录
    eprintln!("Waiting for JRE and JAR to be extracted...");
    extract_bundle(&temp_path);

    // 获取 JRE 和 JAR 文件路径
    let jre_path = temp_path.join("jre/bin/java.exe");
    let jar_path = temp_path.join("app.jar");

    eprintln!("------- Running Java Application -------");

    // 执行 java -jar
    let output = Command::new(jre_path)
        .arg("-jar")
        .arg(jar_path)
        .spawn()
        .expect("Failed to start Java process");

    // 等待进程结束
    let _ = output.wait_with_output();

    eprintln!("------- Java Application Finished -------");

    // 删除临时目录
    fs::remove_dir_all(temp_path).expect("Failed to remove temp directory");
}

fn extract_bundle(dest: &Path) {
    fs::create_dir_all(dest.join("jre")).expect("Failed to create JRE directory");
    let jre = BUNDLE
        .get_dir("jre")
        .expect("JRE not found. Have you included it in the `bundle` dir with the name `jre\\`?");
    jre.extract(dest).expect("Failed to extract JRE directory");
    let jar = BUNDLE
        .get_file("app.jar")
        .expect("JAR not found. Have you included it in the `bundle` dir with the name `app.jar`?");
    fs::write(dest.join("app.jar"), jar.contents()).expect("Failed to extract JAR file");
}
