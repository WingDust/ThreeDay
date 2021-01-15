/*
 * @Author: your name
 * @Date: 2021-01-10 12:45:56
 * @LastEditTime: 2021-01-15 17:15:26
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\firefox\background.ts
 */

// Firefox 的 TypeScript声明文件地址：https://github.com/DefinitelyTyped/DefinitelyTyped/blob/master/types/firefox-webext-browser/index.d.ts
/// <reference path="./index.d.ts" />

let port_firefox = browser.runtime.connectNative("com.my_application");
port_firefox .onMessage.addListener((msg) =>{
    console.log(msg);
})

port_firefox.onDisconnect.addListener((msg)=>{
	console.log("Received: " + JSON.stringify(browser.runtime.lastError));
	console.log("Received: " + JSON.stringify(msg));
});

// browser.runtime.onInstalled.addListener((msg)=>{
//     console.log(msg);
//     port_firefox.postMessage([0,1])
// })

browser.tabs.onCreated.addListener(async function (){
    let all_urls:any[]=[]
    let tabs:browser.tabs.Tab[] = await browser.tabs.query({})
    for (const tab of tabs) {
         all_urls.push(tab.url)
    }
    // console.log(all_urls);
    all_urls.unshift("firefox")
    port_firefox.postMessage(all_urls)
})

browser.tabs.onRemoved.addListener(async function (){
    let all_urls:any[]=[]
    let tabs = await browser.tabs.query({})
    for (const tab of tabs) {
        all_urls.push(tab.url)
    }
    // console.log(all_urls);
    all_urls.unshift("firefox")
    port_firefox.postMessage(all_urls)
})

browser.tabs.onUpdated.addListener(async function (){
    let all_urls:any[]=[]
    let tabs = await browser.tabs.query({})
    for (const tab of tabs) {
         all_urls.push(tab.url)
    }
    // console.log(all_urls);
    all_urls.unshift("firefox")
    port_firefox.postMessage(all_urls)
})