// src/types/account.ts
// 记账项类型（与后端返回数据结构一致）
export interface Item {
  id: number;
  name: string;
  value: number;
  type: string;
  created_at: string;
}

// 分类枚举（统一管理所有分类，避免硬编码）
export const Types = {
  Food: '饭',
  Drink: '饮',
  Shopping: '购',
  Entertainment: '娱',
  Study: '学',
  Transport: '行',
  Service: '充',
  Clothes: '衣',
  Housing: '住',
  Other: '别',
} as const;

// 分类类型（推导Types的value类型，确保类型安全）
export type ItemType = typeof Types[keyof typeof Types];
