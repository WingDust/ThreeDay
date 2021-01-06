<!--
 * @Author: your name
 * @Date: 2020-12-29 13:56:09
 * @LastEditTime: 2021-01-06 16:49:53
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\README.md
-->
## Chrome Extension
  - Reference
    - [What is Native Messaging between applications and how does it work?](https://stackoverflow.com/questions/25169384/what-is-native-messaging-between-applications-and-how-does-it-work)
  - 依赖
    - [chrmod/rust-webextension-protocol](https://github.com/chrmod/rust-webextension-protocol)
    - [serde-rs/serde](https://github.com/serde-rs/serde) 

## Mentaily
  - 首先是在启动 Chrome 时，还是要将所有的 tab 的 url 传送出去，而在 Rust 程序接收到这个信息后，要对这个传入的数组除入反序列化外，还要对这个数组的长度进行检测，以及当前已保存 url 的 backup.json 文件中是否有内容要进行判断 
    1. 当为 0 时
  - 当按了像 OneTab 这样的会大量减少 tabs 的数量，怎么处理？

## Rust
  - Rust `if` `else if` `else`
    > `if` `else if` `else` 在一个函数中使用时，这些语句常常分担了划分界限的功能
    > 而在函数有返回值的情况下，被划分出来的界限中有为正常界限，而总存在与之相反非正常界限