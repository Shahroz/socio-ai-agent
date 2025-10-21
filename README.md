# Socio AI Agent

A comprehensive AI-powered social media content management system built with Rust and React. This agent can generate content for various social media platforms, manage API integrations, and provide real-time chat functionality.

## Features

### 🤖 AI Content Generation
- Generate content for LinkedIn, Twitter, Facebook, Instagram, and emails
- Powered by Google Gemini LLM
- Context-aware responses with conversation memory
- Platform-specific content optimization

### 🔌 Social Media Integration
- Support for multiple social media platforms
- Secure API credential management
- Real-time posting capabilities
- Platform-specific content formatting

### 💬 Real-time Chat Interface
- WebSocket-based real-time communication
- Persistent conversation history
- Responsive and modern UI
- Local storage for session persistence

### 🏗️ Architecture
- **Backend**: Rust with Actix Web framework
- **Frontend**: React with TypeScript, Tailwind CSS, and Vite
- **LLM**: Google Gemini integration
- **Real-time**: WebSocket connections with Actix actors
- **API**: OpenAPI 3.0 specification

## Project Structure

```
socio-ai-agent/
├── crates/
│   ├── llm/              # Gemini LLM integration
│   ├── socio/            # Main backend application
│   │   ├── backend/      # Backend code
│   │   └── frontend/     # React frontend
│   └── agent/            # AI agent orchestration
├── Cargo.toml            # Workspace configuration
└── README.md
```

## Quick Start

### Prerequisites

- Rust 1.70+ 
- Node.js 18+
- Google Gemini API key

### Backend Setup

1. **Clone and navigate to the project:**
   ```bash
   git clone <repository-url>
   cd socio-ai-agent
   ```

2. **Set up environment variables:**
   ```bash
   cp env.example .env
   # Edit .env and add your Gemini API key
   ```

3. **Install dependencies and run:**
   ```bash
   cargo run --bin socio
   ```

The backend will start on `http://localhost:3000`

### Frontend Setup

1. **Navigate to frontend directory:**
   ```bash
   cd crates/socio/frontend
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Start development server:**
   ```bash
   npm run dev
   ```

The frontend will start on `http://localhost:5173`

## Environment Configuration

Create a `.env` file in the project root:

```env
# Server Configuration
PORT=3000
HOST=0.0.0.0

# Gemini API Configuration
GEMINI_API_KEY=your_gemini_api_key_here
GEMINI_MODEL=gemini-1.5-flash

# Social Media API Keys (optional)
LINKEDIN_CLIENT_ID=
LINKEDIN_CLIENT_SECRET=
TWITTER_API_KEY=
TWITTER_API_SECRET=
FACEBOOK_APP_ID=
FACEBOOK_APP_SECRET=
INSTAGRAM_APP_ID=
INSTAGRAM_APP_SECRET=

# Logging
RUST_LOG=info
```

## API Documentation

The API follows OpenAPI 3.0 specification and is automatically generated from Rust code using `utoipa`. Swagger UI is integrated for interactive API exploration:

- **Swagger UI**: `http://localhost:3000/swagger-ui/`
- **OpenAPI JSON**: `http://localhost:3000/api/openapi.json`
- **Health Check**: `http://localhost:3000/api/health`

### Key Endpoints

- `POST /api/chat` - Send messages to the AI agent
- `POST /api/social/config` - Configure social media platforms
- `GET /ws` - WebSocket connection for real-time communication

## Usage

### Chat Interface

1. Open the frontend application
2. Start typing your request in the chat interface
3. The AI agent will respond with relevant content suggestions
4. Use the quick action buttons for common tasks

### Content Generation Examples

- "Generate a LinkedIn post about AI trends"
- "Create a Twitter thread about productivity tips"
- "Write an Instagram caption for a tech photo"
- "Compose a professional email to a client"

### Social Media Configuration

1. Navigate to the settings/configuration section
2. Enter your API credentials for desired platforms
3. The agent will securely store and use these credentials

## Development

### Backend Development

```bash
# Run with hot reload
cargo watch -x run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Frontend Development

```bash
# Generate API types from OpenAPI spec
npm run generate-api

# Run development server
npm run dev

# Build for production
npm run build

# Run linting
npm run lint
```

## Architecture Details

### Backend (Rust)

- **llm crate**: Handles Gemini API integration and content generation
- **socio crate**: Main web server with Actix Web, WebSocket support, social media APIs, and OpenAPI generation
- **agent crate**: Orchestrates AI agent logic, context management, and memory

### Frontend (React)

- **Context Provider**: Manages chat state and WebSocket connections
- **Components**: Modular React components with TypeScript
- **Styling**: Tailwind CSS with custom design system
- **API Integration**: Auto-generated TypeScript clients from OpenAPI spec

### Key Features Implementation

1. **WebSocket Sessions**: Real-time bidirectional communication with Actix actors
2. **Memory Management**: Persistent conversation history with session cleanup
3. **LLM Integration**: Context-aware content generation with Gemini
4. **Social Media APIs**: Extensible platform integration system
5. **Responsive UI**: Mobile-first design with modern UX patterns

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions and support, please open an issue on GitHub or contact the maintainers.
