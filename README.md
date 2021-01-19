<!--
 * @Author: your name
 * @Date: 2020-12-29 13:56:09
 * @LastEditTime: 2021-01-18 21:45:25
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\README.md
-->
## Reason：关于创建这个插件的理由
  -虽然有 FireFox Chrome 本身的恢复，但是还是会遇到一些不能恢复的情况，出现 FireFox Chrome Crash 或者 操作系统 Crash or Reboot 时，这样话我珍贵浏览后却没来得及归档的 Tab 就损失掉了，浪费不少心血。它会保存三天中最新使用 Tab 的浏览记录于本地。
  - 使用 Rust 原因，是为深入 Rust

## 使用
  - Windows：
    - 配置本地程序
      1. 将 TestWeb.exe 和 saveFile.exe 放入 D：盘下的 threeday 文件夹下，没有这个文件夹，就自行创建
      2. 右键以管理员身份运行 saveFile
  - Linux：
    - 配置本地程序
      1. 将 TestWeb 和 saveFile 复制成2份分别各放入 `~/.mozilla/native-messaging-hosts/TestWeb` `~/.config/google-chrome/NativeMessingHosts/TestWeb`
  - 
## Mentaily
  - Chrome 在每一次创建与关闭一个 Tab 时，会将 Chrome 的所有窗口中所有 Tabs 以字符串数组的形式发送给 Rust 程序，再由 Rust 程序写成 JSON 文件
    - 关于保存的间隔与形式为一天中最新的 Tabs 状态将保存成一个 `backup-[].json` 形式文件，最多保留三天的状态。
  - 当我有一天的时间没有使用 Chrome 呢:中间有空档期
    - 超过一天的上一次使用状态仍会被视为上一天的保存状态
## TODO
  - [x] 对这要保存的三天文件进行文件命名固定
    - backup-1.json：今天-现在的使用状态
    - backup-2.json：昨天-上一次的使用状态
    - backup-3.json：前天-上上次的使用状态
  - [x] Extension Test
  - [x] FireFox Extension Learn
  - [x] 插件名
    - Three Day
  - [ ] Linux Browser Test
  - [x] 在同一台电脑下的即使用 Chrome 又使用 FireFox 对它们的保存路径的设置

## Developer Attention
  - 配置文件的位置
    - 暂设定 Windows 为 D：盘
    - Linux 为用户文件
    - 清单文件与应用程序文件应放到同一个文件下
  - 对传入的消息识别处理

## 依赖
  - Rust
    - [chrmod/rust-webextension-protocol](https://github.com/chrmod/rust-webextension-protocol)
    - []()
    - [serde-rs/serde](https://github.com/serde-rs/serde) 
      - Rust 写入 JSON 文件
      ```rust
        let f =File::create("backup-1.json").expect("create backup-1 failed");
      // f.write_all(message.as_bytes()).expect("write file failed");
        let v:Value=serde_json::from_str("[1,2,3]").expect("trans json error");
        // let v1 =serde_json::to_string(&v).unwrap();
        serde_json::to_writer(&f, &v).expect("write json failed");
      ```
      - [serde json 常用实践](https://blog.csdn.net/q435201823/article/details/108038755)
    - [chronotope/chrono](https://github.com/chronotope/chrono)
      - 引入包
      ```rust
      extern crate chrono ;
      use chrono::prelude::*;
      use std::time::SystemTime;//标准库中关于系统时间的库
      ```
      - 获取系统系统本地时间，它包含了时区
      ```rust
      let dt = Local::now();
      println!("dt: {}", dt);
      ```
      - format SystemTime to String
        - [How to format SystemTime to string?](https://stackoverflow.com/questions/45386585/how-to-format-systemtime-to-string)
        - 仅打印日期。与转成日期格式，后设置时、分、秒再打印
        ```rust
        let metadata = fs::metadata("backup.json").expect("error");
        let time = metadata.modified().expect("time error");
        let t: DateTime<Local> = time.into(); //SystemTime 转 DateTime<Local>
        // let t: DateTime<Utc> = time.into();
        let t1 =t.date();//转成日期格式
        println!("{}",t1.format("%d/%m/%Y"));//仅打印日期
        println!("{}",t1.and_hms(0,0,0).format("%d/%m/%Y %T"));//后设置时、分、秒再打印
        ```
      - 如何比较两个时间不是同一天
      ```rust
      let s1 = dt.date().and_hms(0, 0, 0).timestamp();
      let s2 = t.date().and_hms(0, 0, 0).timestamp();
      // 将DateTime<Local>类型转成日期类型，再用 and_hms 转成DateTime<Local> 再用timestamp转成秒进行来比较
      ```

      - [Perform checked date and time calculations](https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html#perform-checked-date-and-time-calculations)


## Chrome Extension
  - 编译 TypeScript
    - `$ tsc`
  - Reference
    - [What is Native Messaging between applications and how does it work?](https://stackoverflow.com/questions/25169384/what-is-native-messaging-between-applications-and-how-does-it-work)
    - [消息传递](https://crxdoc-zh.appspot.com/extensions/messaging#external)
    - Debug Chrome Native Messaging
      - [chrome extension native message & native client](https://blog.csdn.net/weixin_36139431/article/details/98870250)
        - `Chrome.exe --enable-logging`
    - 原生程序的清单文件的 name 中好像不能使用大写

## FireFox Webextension
  - FireFox  Webextension 使用 TypeScript 的声明文件，以及 Edge 的使用 TypeScript 的声明文件
    - [browser.d.ts dependency for Edge web extension APIs browser.runtime.* in typescript](https://stackoverflow.com/questions/43650517/browser-d-ts-dependency-for-edge-web-extension-apis-browser-runtime-in-typescr)
  - 在提交 Add-on 时，manifest.json 中的 version 决定了提交时的版本。

## Github Markdown
  - 在使用有序列表时，第一级会使用阿拉伯数字，第二级会使用罗马数字，第三级会使用英文字母

## Deno
  - [deno 初体验，实战记录一个node项目迁移到deno需要做什么](https://cloud.tencent.com/developer/article/1640293)
  - [Read a json file in Deno](https://www.seanmcp.com/articles/read-a-json-file-in-deno/)
## Rust
  - Rust `if` `else if` `else`
    > `if` `else if` `else` 在一个函数中使用时，这些语句常常分担了划分界限的功能
    > 而在函数有返回值的情况下，被划分出来的界限中有为正常界限，而总存在与之相反非正常界限
  - RustDoc
    - [What is rustdoc?](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
    - [【Rust日报】2021-01-14 rustdoc的性能有了很大的提升！](https://rustcc.cn/article?id=f27b49b2-7803-4011-9f4d-c924c152a0ab)
  - 交叉编译
    - 放弃
  - 条件编译

## 发布
  - Chrome Extension 发布需要以五美元注册开发者

## License
  MIT LICENSE