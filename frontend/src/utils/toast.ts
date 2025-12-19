import type { ToastItem } from '@/components/error';

// 全局 toast 容器引用
let toastContainer: any = null;

/**
 * 设置全局 toast 容器实例
 */
export function setToastContainer(container: any) {
  toastContainer = container;
}

/**
 * 显示错误提示
 */
export function showError(message: string, title?: string, duration = 4000) {
  if (!toastContainer) {
    console.error('Toast container not initialized:', message);
    return;
  }
  toastContainer.addToast({
    type: 'error',
    title: title || '错误',
    message,
    duration,
    closable: true,
    showIcon: true,
  });
}

/**
 * 显示警告提示
 */
export function showWarning(message: string, title?: string, duration = 3000) {
  if (!toastContainer) {
    console.warn('Toast container not initialized:', message);
    return;
  }
  toastContainer.addToast({
    type: 'warning',
    title: title || '警告',
    message,
    duration,
    closable: true,
    showIcon: true,
  });
}

/**
 * 显示成功提示
 */
export function showSuccess(message: string, title?: string, duration = 3000) {
  if (!toastContainer) {
    console.log('Toast container not initialized:', message);
    return;
  }
  toastContainer.addToast({
    type: 'success',
    title: title || '成功',
    message,
    duration,
    closable: true,
    showIcon: true,
  });
}

/**
 * 显示信息提示
 */
export function showInfo(message: string, title?: string, duration = 3000) {
  if (!toastContainer) {
    console.info('Toast container not initialized:', message);
    return;
  }
  toastContainer.addToast({
    type: 'info',
    title: title || '提示',
    message,
    duration,
    closable: true,
    showIcon: true,
  });
}

/**
 * 显示确认对话框（使用 warning 类型的 toast，不会自动关闭）
 * 注意：这是一个简化版本，不能真正阻塞代码执行，需要配合回调使用
 */
export function showConfirm(
  message: string,
  onConfirm: () => void,
  onCancel?: () => void,
  title = '确认'
) {
  // 由于 toast 不是真正的模态对话框，这里暂时使用浏览器原生 confirm
  // 未来可以创建一个专门的 ConfirmDialog 组件
  const result = confirm(`${title}\n\n${message}`);
  if (result) {
    onConfirm();
  } else if (onCancel) {
    onCancel();
  }
  return result;
}

/**
 * 清空所有提示
 */
export function clearAllToasts() {
  if (toastContainer) {
    toastContainer.clearAll();
  }
}
