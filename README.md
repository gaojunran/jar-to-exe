# jar-to-exe

这是一个简易的，将JAR包打包成Windows平台可执行文件（.exe）的工具。

## 局限性

此项目的起源是我的一门课程需要以这种方式提交作业（是的，它需要每个人捆绑一个JRE来提交，完全舍弃了Java“一次编写、多端运行”的优势），所以此项目仅在以下条件下适用：

- 仅支持Windows平台。
- 你编写的程序可以简单地使用`java -jar`命令来运行，不依赖任何其它命令行参数。

## 使用方法

1. 安装 Rust 编译工具（[教程](https://www.rust-lang.org/zh-CN/tools/install)）

2. 将你的Java或Kotlin项目打包成一个JAR文件，最常见的做法是使用Maven、Gradle；或者使用IntelliJ IDEA的“构建工件”功能。

> [!TIP]
> 如果你的代码中包含资源文件，应该在此步骤中打包进JAR文件。

3. 克隆此仓库，将JAR文件（命名为`app.jar`）与JRE目录（命名为`jre/`）一起放在此项目的`bundle/`路径下。

4. 运行`cargo build --release`编译。
