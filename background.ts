/*
 * @Author: your name
 * @Date: 2020-12-25 15:07:48
 * @LastEditTime: 2020-12-25 15:08:08
 * @LastEditors: your name
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\background.ts
 */

// chrome 的 TypeScript声明文件地址：https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/chrome
/// <reference path="./index.d.ts" />

chrome.tabs.query({currentWindow:true},function(){
    console.log(12312);
})