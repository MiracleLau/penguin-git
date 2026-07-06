import type { MessageApiInjection } from "naive-ui/es/message/src/MessageProvider";

let messageApi: MessageApiInjection | null = null;

export function setMessageApi(api: MessageApiInjection) {
  messageApi = api;
}

export function showSuccess(msg: string) {
  messageApi?.success(msg, { duration: 3000 });
}

export function showDanger(msg: string) {
  messageApi?.error(msg, { duration: 5000 });
}

export function showWarning(msg: string) {
  messageApi?.warning(msg, { duration: 4000 });
}

export function showInfo(msg: string) {
  messageApi?.info(msg, { duration: 3000 });
}
