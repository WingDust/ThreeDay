/*
 * @Author: your name
 * @Date: 2021-01-10 12:45:56
 * @LastEditTime: 2021-01-11 21:36:49
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\firefox\background.ts
 */

// Firefox 的 TypeScript声明文件地址：https://github.com/DefinitelyTyped/DefinitelyTyped/blob/master/types/firefox-webext-browser/index.d.ts
/// <reference path="./index.d.ts" />

let port_firefox = browser.runtime.connectNative('com.my_company.my_application');
port_firefox .onMessage.addListener(function(msg){
    console.log(msg);
})


browser.tabs.onCreated.addListener(async function (){
    let all_urls:any[]=[]
    let tabs:browser.tabs.Tab[] = await browser.tabs.query({})
    for (const tab of tabs) {
        all_urls = all_urls.concat(tab.url)
    }
    console.log(all_urls);
    port_firefox.postMessage(all_urls)
})


browser.tabs.onRemoved.addListener(async function (){
    let all_urls:any[]=[]
    let tabs = await browser.tabs.query({})
    for (const tab of tabs) {
        all_urls.push(tab.url)
    }
    console.log(all_urls);
    port_firefox.postMessage(all_urls)
})

browser.tabs.onUpdated.addListener(async function (){
    let all_urls:any[]=[]
    let tabs = await browser.tabs.query({})
    for (const tab of tabs) {
        all_urls = all_urls.concat(tab.url)
    }
    console.log(all_urls);
    port_firefox.postMessage(all_urls)
})