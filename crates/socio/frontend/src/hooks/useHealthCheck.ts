import { useState, useEffect } from 'react'
import { HealthService, HealthResponse } from '../api'

export interface UseHealthCheckReturn {
  health: HealthResponse | null
  isLoading: boolean
  error: string | null
  checkHealth: () => Promise<void>
}

export function useHealthCheck(): UseHealthCheckReturn {
  const [health, setHealth] = useState<HealthResponse | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const checkHealth = async (): Promise<void> => {
    setIsLoading(true)
    setError(null)

    try {
      const response = await HealthService.healthCheck()
      setHealth(response)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Health check failed'
      setError(errorMessage)
    } finally {
      setIsLoading(false)
    }
  }

  // Auto-check health on mount
  useEffect(() => {
    checkHealth()
  }, [])

  return {
    health,
    isLoading,
    error,
    checkHealth,
  }
}
