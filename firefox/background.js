"use strict";
/*
 * @Author: your name
 * @Date: 2021-01-10 12:45:56
 * @LastEditTime: 2021-01-10 16:16:07
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\firefox\background.ts
 */
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
// Firefox 的 TypeScript声明文件地址：https://github.com/DefinitelyTyped/DefinitelyTyped/blob/master/types/firefox-webext-browser/index.d.ts
/// <reference path="./index.d.ts" />
browser.tabs.onCreated.addListener(function () {
    return __awaiter(this, void 0, void 0, function* () {
        let all_urls = [];
        let tabs = yield browser.tabs.query({});
        console.log(tabs);
        // let windows = await browser.windows.getAll()
        // console.log(windows);
    });
});
browser.tabs.onRemoved.addListener(function () {
    return __awaiter(this, void 0, void 0, function* () {
        let all_urls = [];
        let windows = yield browser.windows.getAll();
        console.log(windows);
    });
});
browser.tabs.onUpdated.addListener(function () {
    return __awaiter(this, void 0, void 0, function* () {
        let all_urls = [];
        let windows = yield browser.windows.getAll();
        console.log(windows);
    });
});
