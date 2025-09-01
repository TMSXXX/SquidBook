// src/utils/dateUtil.ts
/**
 * 日期格式转换：把 "2024-05-20T12:34:56" 转为 "2024-05-20"（去掉时间部分）
 * @param dateStr - 带时间的日期字符串
 * @returns 仅日期部分的字符串
 */
export const dateConvert = (dateStr: string): string => {
  if (!dateStr) return '';
  const splitResult = dateStr.split('T');
  return splitResult[0] || dateStr;
};

/**
 * 获取今日日期（格式：YYYY-MM-DD）
 * @returns 今日日期字符串
 */
export const getToday = (): string => {
  return dateConvert(new Date().toISOString());
};
