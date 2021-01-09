/*
 * @Author: your name
 * @Date: 2021-01-09 15:51:23
 * @LastEditTime: 2021-01-09 15:52:01
 * @LastEditors: your name
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\har-format\index.d.ts
 */
import { Entry, Log } from 'har-format';

declare global {
    export type HARFormatEntry = Entry;
    export type HARFormatLog = Log;
}
