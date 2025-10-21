import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { ChatProvider } from './contexts/ChatContext'
import ChatInterface from './components/ChatInterface'
import { ApiDemo } from './components/ApiDemo'

function App() {
  return (
    <ChatProvider>
      <Router>
        <div className="min-h-screen bg-gray-50">
          <Routes>
            <Route path="/" element={<ChatInterface />} />
            <Route path="/api-demo" element={<ApiDemo />} />
          </Routes>
        </div>
      </Router>
    </ChatProvider>
  )
}

export default App
