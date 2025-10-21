import React, { createContext, useContext, useReducer, useEffect } from 'react'
import { ChatService, ChatRequest, ChatResponse } from '../api'
import { convertApiMessageToLocal } from '../utils/messageConverter'

export interface ChatMessage {
  id: string
  content: string
  timestamp: Date
  sender: 'user' | 'assistant'
}

export interface ChatState {
  messages: ChatMessage[]
  sessionId: string | null
  isConnected: boolean
  isLoading: boolean
}

type ChatAction =
  | { type: 'ADD_MESSAGE'; payload: ChatMessage }
  | { type: 'SET_SESSION_ID'; payload: string }
  | { type: 'SET_CONNECTION_STATUS'; payload: boolean }
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'CLEAR_MESSAGES' }
  | { type: 'LOAD_MESSAGES'; payload: ChatMessage[] }

const initialState: ChatState = {
  messages: [],
  sessionId: null,
  isConnected: false,
  isLoading: false,
}

function chatReducer(state: ChatState, action: ChatAction): ChatState {
  switch (action.type) {
    case 'ADD_MESSAGE':
      return {
        ...state,
        messages: [...state.messages, action.payload],
      }
    case 'SET_SESSION_ID':
      return {
        ...state,
        sessionId: action.payload,
      }
    case 'SET_CONNECTION_STATUS':
      return {
        ...state,
        isConnected: action.payload,
      }
    case 'SET_LOADING':
      return {
        ...state,
        isLoading: action.payload,
      }
    case 'CLEAR_MESSAGES':
      return {
        ...state,
        messages: [],
        sessionId: null,
      }
    case 'LOAD_MESSAGES':
      return {
        ...state,
        messages: action.payload,
      }
    default:
      return state
  }
}

interface ChatContextType {
  state: ChatState
  dispatch: React.Dispatch<ChatAction>
  sendMessage: (message: string) => void
  clearChat: () => void
}

const ChatContext = createContext<ChatContextType | undefined>(undefined)

export function ChatProvider({ children }: { children: React.ReactNode }) {
  const [state, dispatch] = useReducer(chatReducer, initialState)

  // Load messages from localStorage on mount
  useEffect(() => {
    const savedMessages = localStorage.getItem('socio-chat-messages')
    const savedSessionId = localStorage.getItem('socio-session-id')
    
    if (savedMessages) {
      try {
        const messages = JSON.parse(savedMessages).map((msg: any) => ({
          ...msg,
          timestamp: new Date(msg.timestamp)
        }))
        dispatch({ type: 'LOAD_MESSAGES', payload: messages })
      } catch (error) {
        console.error('Failed to load saved messages:', error)
      }
    }
    
    if (savedSessionId) {
      dispatch({ type: 'SET_SESSION_ID', payload: savedSessionId })
    }
  }, [])

  // Save messages to localStorage whenever messages change
  useEffect(() => {
    if (state.messages.length > 0) {
      localStorage.setItem('socio-chat-messages', JSON.stringify(state.messages))
    }
  }, [state.messages])

  // Save session ID to localStorage
  useEffect(() => {
    if (state.sessionId) {
      localStorage.setItem('socio-session-id', state.sessionId)
    }
  }, [state.sessionId])

  const sendMessage = async (message: string) => {
    if (!message.trim()) return

    const userMessage: ChatMessage = {
      id: Date.now().toString(),
      content: message,
      timestamp: new Date(),
      sender: 'user',
    }

    dispatch({ type: 'ADD_MESSAGE', payload: userMessage })
    dispatch({ type: 'SET_LOADING', payload: true })

    try {
      const request: ChatRequest = {
        message,
        session_id: state.sessionId || undefined,
      }

      const response: ChatResponse = await ChatService.handleChat({
        requestBody: request,
      })
      
      const assistantMessage: ChatMessage = convertApiMessageToLocal(response.message)

      dispatch({ type: 'ADD_MESSAGE', payload: assistantMessage })
      
      if (!state.sessionId) {
        dispatch({ type: 'SET_SESSION_ID', payload: response.session_id })
      }
    } catch (error) {
      console.error('Error sending message:', error)
      const errorMessage: ChatMessage = {
        id: Date.now().toString(),
        content: 'Sorry, I encountered an error. Please try again.',
        timestamp: new Date(),
        sender: 'assistant',
      }
      dispatch({ type: 'ADD_MESSAGE', payload: errorMessage })
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false })
    }
  }

  const clearChat = () => {
    dispatch({ type: 'CLEAR_MESSAGES' })
    localStorage.removeItem('socio-chat-messages')
    localStorage.removeItem('socio-session-id')
  }

  return (
    <ChatContext.Provider value={{ state, dispatch, sendMessage, clearChat }}>
      {children}
    </ChatContext.Provider>
  )
}

export function useChat() {
  const context = useContext(ChatContext)
  if (context === undefined) {
    throw new Error('useChat must be used within a ChatProvider')
  }
  return context
}
