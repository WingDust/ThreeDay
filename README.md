<!--
 * @Author: your name
 * @Date: 2020-12-29 13:56:09
 * @LastEditTime: 2021-01-15 10:41:20
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\README.md
-->
## Reason:关于创建这个插件的理由
  - 当出现 Chrome Crash 或者 操作系统 Crash 时，虽然在这二个情况下有 Chrome 本身的恢复，但是我还是会遇到一些不能恢复的情况，这样话我珍贵浏览后却没来得及归档的 Tab 就损失掉了。
  - 使用 Rust 原因，是为深入 Rust

## 使用
  1 . 如果将 TestWeb.exe 
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
  - [ ] 在同一台电脑下的即使用 Chrome 又使用 FireFox 对它们的保存路径的设置

## Developer Attention
  - 配置文件的位置
    - 暂设定 Windows 为 D：盘
    - Linux 为用户文件
    - 清单文件与应用程序文件应放到同一个文件下
  - 对传入的消息识别处理
    - 当扩展被第一次安装时，或更新时会传入信息，表明扩展已安装，用来继续后面的配置
      - Chrome  为 `[1，0]`
      - FireFox 为 `[0, 1]`

## 依赖
  - Rust
    - [chrmod/rust-webextension-protocol](https://github.com/chrmod/rust-webextension-protocol)
    - [serde-rs/serde](https://github.com/serde-rs/serde) 
    - [chronotope/chrono](https://github.com/chronotope/chrono)

## Chrome Extension
  - 编译 TypeScript
    - `$ tsc`
  - Reference
    - [What is Native Messaging between applications and how does it work?](https://stackoverflow.com/questions/25169384/what-is-native-messaging-between-applications-and-how-does-it-work)
    - [消息传递](https://crxdoc-zh.appspot.com/extensions/messaging#external)
    - Debug Chrome Native Messaging
      - [chrome extension native message & native client](https://blog.csdn.net/weixin_36139431/article/details/98870250)

## FireFox Webextension
  - FireFox  Webextension 使用 TypeScript 的声明文件，以及 Edge 的使用 TypeScript 的声明文件
    - [browser.d.ts dependency for Edge web extension APIs browser.runtime.* in typescript](https://stackoverflow.com/questions/43650517/browser-d-ts-dependency-for-edge-web-extension-apis-browser-runtime-in-typescr)
## Rust
  - Rust `if` `else if` `else`
    > `if` `else if` `else` 在一个函数中使用时，这些语句常常分担了划分界限的功能
    > 而在函数有返回值的情况下，被划分出来的界限中有为正常界限，而总存在与之相反非正常界限
  - RustDoc
    - [What is rustdoc?](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
    - [【Rust日报】2021-01-14 rustdoc的性能有了很大的提升！](https://rustcc.cn/article?id=f27b49b2-7803-4011-9f4d-c924c152a0ab)