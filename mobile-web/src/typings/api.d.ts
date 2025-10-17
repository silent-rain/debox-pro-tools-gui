// 接口

// 提交提示词
export interface AddPromptReq {
  style: number;
  batch_size: number;
  width: number;
  height: number;
  positive: string;
  negative: string;
}

export interface AddPromptRsp {
  prompt_id: string;
  number: number;
  node_errors: any;
}

// 查看历史记录
export interface HistoryReq {
  prompt_id: string;
}

export interface HistoryRsp {
  filename: string;
  type: string;
  subfolder: string;
}

// 查看队列
export interface QueueReq {
  prompt_id: string;
}

export interface QueueRsp {
  queue_pending_index: number;
  queue_pending_total: number;
  status: number;
}

// 查看图片
export interface ViewReq {
  filename: string;
  type: string;
  subfolder: string;
}
