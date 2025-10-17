import { AddPromptReq, AddPromptRsp, HistoryReq, HistoryRsp, QueueReq, QueueRsp, ViewReq } from '@/typings/api';
import request from '@/utils/request';
import { invoke } from '@tauri-apps/api/core';
import { server } from './constant';

export async function greet(data: string) {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke('greet', { data });
}

export const comfyuiAddPrompt = async (data: AddPromptReq): Promise<AddPromptRsp> => {
  return await invoke('comfyui_add_prompt', { data });
};

export const comfyuiHistory = async (data: HistoryReq): Promise<HistoryRsp[]> => {
  return await invoke('comfyui_history', { data });
};

export const comfyuiQueue = async (data: QueueReq): Promise<QueueRsp> => {
  return await invoke('comfyui_queue', { data });
};
export const comfyuiView = async (data: ViewReq): Promise<any> => {
  return await invoke('comfyui_view', { data });
};

export const hello = (data: any): Promise<any> => {
  return request({
    url: `${server}/hello`,
    method: 'GET',
    params: data,
  });
};

/*
websocket 插件在 JavaScript 中可用。

import WebSocket from '@tauri-apps/plugin-websocket';
// when using `"withGlobalTauri": true`, you may use
// const WebSocket = window.__TAURI__.websocket;

const ws = await WebSocket.connect('ws://127.0.0.1:8080');

ws.addListener((msg) => {
  console.log('Received Message:', msg);
});

await ws.send('Hello World!');

await ws.disconnect();
*/
