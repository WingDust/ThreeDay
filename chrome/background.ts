// deno-lint-ignore-file
/*
 * @Author: your name
 * @Date: 2020-12-25 15:07:48
 * @LastEditTime: 2021-01-15 11:15:06
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\background.ts
 */

// chrome 的 TypeScript声明文件地址：https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/chrome
/// <reference path="./index.d.ts" />


// deno-lint-ignore no-undef
let port = chrome.runtime.connectNative('chrome_nativeMessaging');
port.onMessage.addListener(function(msg){
    console.log(msg);
})
// port.postMessage("[{as:1},2,3]");

// chrome.runtime.onInstalled.addListener(()=>{
//     port.postMessage([1,0])
// })
/**
* 发出的数据都应该是字符串数组
*/
chrome.tabs.onRemoved.addListener(function () {
    // chrome.tabs.query({currentWindow:true},function(tabs){
    //     console.log(tabs);
    // })
    let all_urls:any[]=[]
    chrome.windows.getAll({populate:true},function (windows:chrome.windows.Window[]) {
        if (windows.length ==1) {//当窗口为 1 个的时候
        // console.log(windows[0].tabs![0].url)
        for (const tab of windows[0].tabs!) {
            all_urls.push(tab.url)
        }
        console.log(JSON.stringify(all_urls));
        port.postMessage(JSON.stringify(all_urls))
        }
        else{ //当窗口不为 1 个的时候
            let all_tabs:any[]=[];
            for (const window of windows) {//forof 遍历值 forin 遍历key
                 all_tabs = all_tabs.concat(window.tabs)
            }
            for (const tab of all_tabs) {
                all_urls.push(tab.url)
            }
            all_urls.unshift("chrome")
            console.log(JSON.stringify(all_urls))
            port.postMessage(JSON.stringify(all_urls))
        }
    })

    // [The chrome.fileSystem API is available to Chrome apps, but not to Chrome extensions]https://javascriptio.com/view/4907510/uncaught-typeerror-cannot-read-property-chooseentry-of-undefined-while-developing-a-chrome-extension-using-filesystem-api)
    // chrome.fileSystem.chooseEntry({type:'openDirectory'},function(Entry:any){
    //     console.log(Entry);
    //     Entry.getDirectory('new_folder',{create:true},function (subEntry:any) {
    //         console.log(subEntry);
    //     })
    // })
})

chrome.tabs.onUpdated.addListener(function () {
    let all_urls:any[]=[]
    chrome.windows.getAll({populate:true},function (windows:chrome.windows.Window[]) {
        if (windows.length ==1) {//当窗口为 1 个的时候
        // console.log(windows[0].tabs![0].url)
        for (const tab of windows[0].tabs!) {
            all_urls.push(tab.url)
        }
        console.log(JSON.stringify(all_urls));
        port.postMessage(JSON.stringify(all_urls))
        }
        else{ //当窗口不为 1 个的时候
            let all_tabs:any[]=[];
            for (const window of windows) {//forof 遍历值 forin 遍历key
                 all_tabs = all_tabs.concat(window.tabs)
            }
            for (const tab of all_tabs) {
                all_urls.push(tab.url)
            }
            all_urls.unshift("chrome")
            console.log(JSON.stringify(all_urls))
            port.postMessage(JSON.stringify(all_urls))
        }
    })
})

chrome.tabs.onCreated.addListener(function () {
    let all_urls:any[]=[]
    chrome.windows.getAll({populate:true},function (windows:chrome.windows.Window[]) {
        if (windows.length ==1) {//当窗口为 1 个的时候
        // console.log(windows[0].tabs![0].url)
        for (const tab of windows[0].tabs!) {
            all_urls.push(tab.url)
        }
        console.log(JSON.stringify(all_urls));
        port.postMessage(JSON.stringify(all_urls))
        }
        else{ //当窗口不为 1 个的时候
            let all_tabs:any[]=[];
            for (const window of windows) {//forof 遍历值 forin 遍历key
                 all_tabs = all_tabs.concat(window.tabs)
            }
            for (const tab of all_tabs) {
                all_urls.push(tab.url)
            }
            all_urls.unshift("chrome")
            console.log(JSON.stringify(all_urls))
            port.postMessage(JSON.stringify(all_urls))
        }
    })
})


// chrome.runtime.onStartup.addListener(function(){
//     let port = chrome.runtime.connectNative('com.my_company.my_application');

// })


// chrome.runtime
// chrome.app.runtime.onLaunched.addListener(function(){
    // console.log('launch');
// })




// chrome.fileSystemProvider