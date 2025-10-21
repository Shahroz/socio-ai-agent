import { useState } from 'react'
import { SocialService, SocialMediaConfig, ConfigResponse } from '../api'

export interface UseSocialConfigReturn {
  configurePlatform: (config: SocialMediaConfig) => Promise<ConfigResponse>
  isLoading: boolean
  error: string | null
}

export function useSocialConfig(): UseSocialConfigReturn {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const configurePlatform = async (config: SocialMediaConfig): Promise<ConfigResponse> => {
    setIsLoading(true)
    setError(null)

    try {
      const response = await SocialService.handleSocialConfig({
        requestBody: config,
      })
      return response
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to configure platform'
      setError(errorMessage)
      throw err
    } finally {
      setIsLoading(false)
    }
  }

  return {
    configurePlatform,
    isLoading,
    error,
  }
}
