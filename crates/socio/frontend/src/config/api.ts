import { OpenAPI } from '../api'

// Configure the OpenAPI client
OpenAPI.BASE = 'http://localhost:3000'
OpenAPI.WITH_CREDENTIALS = false

console.log('API client configured with base URL:', OpenAPI.BASE)
