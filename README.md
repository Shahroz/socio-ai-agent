# Socio AI Agent

A comprehensive AI-powered social media content creation and management platform built with Rust and TypeScript.

## Overview

Socio AI Agent is designed to help users create, manage, and post content across multiple social media platforms using AI assistance. The platform provides intelligent content generation, platform-specific optimization, and seamless posting capabilities.

## Architecture

The project follows a modular architecture with clear separation of concerns:

### Backend (Rust)
- **`crates/socio-backend/`** - Main backend application
- **`crates/agentloop/`** - Agent loop framework for AI tool execution
- **`crates/llm/`** - Large Language Model integration (Gemini)
- **`crates/api_clients/`** - External API client integrations

### Frontend (TypeScript)
- **`crates/socio/frontend/`** - React-based web interface
- Generated API client from OpenAPI specifications

## Key Features

### 🤖 AI Agent Tools

#### 1. GenerateContentTool
Generates social media content using AI with platform-specific optimization:

**Request Parameters:**
- `topic` - Content subject/topic
- `platform` - Target social media platform (linkedin, twitter, facebook, instagram)
- `content_type` - Type of content (post, story, caption, etc.)
- `tone` - Content tone (professional, casual, friendly, etc.)
- `audience` - Target audience description
- `max_length` - Maximum character limit
- `include_hashtags` - Whether to include hashtags
- `include_emojis` - Whether to include emojis
- `context` - Additional context or requirements

**Response:**
- Generated content optimized for the platform
- Platform-specific formatting
- Character count and limit validation
- Suggested hashtags
- Generation metadata

#### 2. SocialMediaPostTool
Posts content to social media platforms with platform-specific handlers:

**Supported Platforms:**
- **LinkedIn** - Professional networking content
- **Twitter/X** - Short-form content with character limits
- **Facebook** - General social content
- **Instagram** - Visual content with image requirements

**Request Parameters:**
- `platform` - Target platform
- `content` - Content to post
- `platform_config` - Platform-specific API credentials
- `image_url` - Optional image attachment
- `scheduled_time` - Optional scheduling
- `publish_immediately` - Immediate or draft posting

**Response:**
- Post success status
- Platform-specific post ID
- Post URL (if available)
- Platform metadata

### 🔧 Platform Configuration

Each social media platform requires specific API credentials:

- **LinkedIn**: Access token
- **Twitter/X**: API key and secret
- **Facebook**: Access token
- **Instagram**: Access token (requires image/video)

### 🌐 API Endpoints

- `POST /api/chat` - Chat with AI agent
- `GET /api/health` - Health check
- `POST /api/social/config` - Configure social media platforms
- `WS /ws` - WebSocket for real-time communication

## Development

### Prerequisites

- Rust 1.70+
- Node.js 18+
- PostgreSQL (for future database features)

### Quick Start

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd socio-ai-agent
   ```

2. **Set up environment**
   ```bash
   cp .env-example .env
   # Edit .env with your configuration
   ```

3. **Run the backend**
   ```bash
   cd crates/socio/backend
   cargo run
   ```

4. **Run the frontend**
   ```bash
   cd crates/socio/frontend
   npm install
   npm run dev
   ```

### Building

```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p socio-backend

# Run tests
cargo test --workspace
```

## Project Structure

```
socio-ai-agent/
├── crates/
│   ├── socio-backend/          # Main backend application
│   │   ├── src/
│   │   │   ├── agent_tools/    # AI agent tool implementations
│   │   │   ├── routes/         # API route handlers
│   │   │   ├── services/       # Business logic services
│   │   │   ├── types/         # Type definitions
│   │   │   └── openapi.rs     # OpenAPI specification
│   │   └── Cargo.toml
│   ├── agentloop/              # Agent loop framework
│   ├── llm/                    # LLM integration
│   ├── agent/                  # AI agent implementation
│   └── api_clients/           # External API clients
├── build/                      # Build configurations
└── docs/                       # Documentation
```

## Coding Standards

The project follows strict coding standards:

- **One-item-per-file**: Each file contains exactly one primary logical item
- **Fully qualified paths**: No `use` statements, all paths are fully qualified
- **Comprehensive documentation**: Every module, function, and struct is documented
- **Functional programming style**: Immutable data and pure functions preferred
- **Comprehensive testing**: Unit tests for all modules

## Contributing

1. Follow the established coding standards
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Ensure all tests pass before submitting PRs

## License

[License information]

## Support

For questions and support, please contact the development team.