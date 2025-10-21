import { ChatMessage as ApiChatMessage } from '../api'
import { ChatMessage } from '../contexts/ChatContext'

/**
 * Convert API ChatMessage to local ChatMessage
 */
export function convertApiMessageToLocal(apiMessage: ApiChatMessage): ChatMessage {
  return {
    id: apiMessage.id,
    content: apiMessage.content,
    timestamp: new Date(apiMessage.timestamp),
    sender: apiMessage.sender as 'user' | 'assistant',
  }
}

/**
 * Convert local ChatMessage to API ChatMessage
 */
export function convertLocalMessageToApi(localMessage: ChatMessage): ApiChatMessage {
  return {
    id: localMessage.id,
    content: localMessage.content,
    timestamp: localMessage.timestamp.toISOString(),
    sender: localMessage.sender,
  }
}
