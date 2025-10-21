import React from 'react'
import { useHealthCheck } from '../hooks/useHealthCheck'
import { useSocialConfig } from '../hooks/useSocialConfig'
import { SocialMediaConfig } from '../api'

export const ApiDemo: React.FC = () => {
  const { health, isLoading: healthLoading, error: healthError, checkHealth } = useHealthCheck()
  const { configurePlatform, isLoading: configLoading, error: configError } = useSocialConfig()

  const handleConfigureLinkedIn = async () => {
    try {
      const config: SocialMediaConfig = {
        platform: 'linkedin',
        api_key: 'test_key',
        api_secret: 'test_secret',
      }
      
      const result = await configurePlatform(config)
      console.log('Configuration result:', result)
    } catch (error) {
      console.error('Configuration failed:', error)
    }
  }

  return (
    <div className="p-4 space-y-4">
      <h2 className="text-xl font-bold">API Demo</h2>
      
      {/* Health Check */}
      <div className="border p-4 rounded">
        <h3 className="font-semibold">Health Check</h3>
        {healthLoading && <p>Checking health...</p>}
        {healthError && <p className="text-red-500">Error: {healthError}</p>}
        {health && (
          <div>
            <p>Status: {health.status}</p>
            <p>Version: {health.version}</p>
            <p>Timestamp: {new Date(health.timestamp).toLocaleString()}</p>
          </div>
        )}
        <button 
          onClick={checkHealth}
          className="mt-2 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Check Health
        </button>
      </div>

      {/* Social Media Configuration */}
      <div className="border p-4 rounded">
        <h3 className="font-semibold">Social Media Configuration</h3>
        {configLoading && <p>Configuring platform...</p>}
        {configError && <p className="text-red-500">Error: {configError}</p>}
        <button 
          onClick={handleConfigureLinkedIn}
          className="mt-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        >
          Configure LinkedIn
        </button>
      </div>
    </div>
  )
}
