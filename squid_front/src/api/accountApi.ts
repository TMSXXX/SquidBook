// src/api/accountApi.ts
import axios from 'axios';
import type { Item, ItemType } from '@/types/account'; // @ 是src目录的别名（需配置）
 // @ 是src目录的别名（需配置）
import { dateConvert } from '@/utils/dateUtil';

// 基础URL（如果后端接口有统一前缀，可在这里配置）
axios.defaults.baseURL = '/api'; // 对应后端的 /api/items 等接口

// 1. 获取所有记账项
export const getItems = async (): Promise<Item[]> => {
  const response = await axios.get<Item[]>('/items');
  // 统一处理日期格式（避免每个组件都要转换）
  return response.data.map(item => ({
    ...item,
    created_at: dateConvert(item.created_at),
  }));
};

// 2. 添加记账项
export interface AddItemParams {
  name: string;
  value: number;
  type: ItemType;
  created_at: string;
}
export const addItem = async (params: AddItemParams): Promise<Item> => {
  const response = await axios.post<Item>('/items', params);
  return response.data;
};

// 3. 删除记账项
export const deleteItem = async (id: number): Promise<void> => {
  await axios.delete(`/items/${id}`);
};

// 4. 更新记账项
export interface UpdateItemParams {
  id: number;
  name: string;
  value: number;
  type: ItemType;
  created_at: string;
}
export const updateItem = async (params: UpdateItemParams): Promise<Item> => {
  const response = await axios.put<Item>(`/items/${params.id}`, params);
  return response.data;
};

// 5. （可选）后端汇总接口（未来切换后端汇总时用）
export interface DailySummary {
  date: string;
  total: number;
}
export const getDailySummary = async (): Promise<DailySummary[]> => {
  const response = await axios.get<DailySummary[]>('/summary/daily');
  return response.data;
};
