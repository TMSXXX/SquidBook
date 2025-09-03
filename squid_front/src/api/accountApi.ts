// src/api/accountApi.ts
import { invoke } from '@tauri-apps/api/core';
import type { Item, ItemType } from '@/types/account';
import { dateConvert } from '@/utils/dateUtil';

// 1. 获取所有记账项
export const getItems = async (): Promise<Item[]> => {
  try {
    const items = await invoke<Item[]>('get_items');
    return items.map(item => ({
      ...item,
      created_at: dateConvert(item.created_at),
    }));
  } catch (error) {
    console.error('获取记账项失败:', error);
    throw error;
  }
};

// 2. 添加记账项
export interface AddItemParams {
  name: string;
  value: number;
  type: ItemType;
  created_at: string;
}

export const addItem = async (params: AddItemParams): Promise<number> => {
  try {
    return await invoke<number>('add_item', {
      request: {
        name: params.name,
        value: params.value,
        item_type: params.type,
        created_at: params.created_at
      }
    });
  } catch (error) {
    console.error('添加记账项失败:', error);
    throw error;
  }
};

// 3. 删除记账项 - 修正参数名
export const deleteItem = async (id: number): Promise<void> => {
  try {
    await invoke('delete_item', { id: id }); // 修改为 id
  } catch (error) {
    console.error('删除记账项失败:', error);
    throw error;
  }
};

// 4. 更新记账项 - 修正参数名
export interface UpdateItemParams {
  id: number;
  name: string;
  value: number;
  type: string;
  created_at: string;
}


export const updateItem = async (params: UpdateItemParams): Promise<void> => {
  try {
    console.log('调用 update_item 参数:', {
      id: params.id,
      name: params.name,
      value: params.value,
      itemType: params.type, // 修正这里
      createdAt: params.created_at
    });
    
    const result = await invoke('update_item', {
      id: params.id,
      name: params.name,
      value: params.value,
      itemType: params.type, // 修正这里
      createdAt: params.created_at
    });
    
    console.log('更新成功:', result);
  } catch (error) {
    console.error('更新记账项失败:', error);
    throw error;
  }
};