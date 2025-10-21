import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { ChatProvider } from './contexts/ChatContext'
import ChatInterface from './components/ChatInterface'
import './App.css'

function App() {
  return (
    <ChatProvider>
      <Router>
        <div className="min-h-screen bg-gray-50">
          <Routes>
            <Route path="/" element={<ChatInterface />} />
          </Routes>
        </div>
      </Router>
    </ChatProvider>
  )
}

export default App
