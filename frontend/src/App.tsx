import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import './App.css'
import MainDisplay from './components/MainDisplay/MainDisplay'
import React from 'react'

const queryClient = new QueryClient()

function App() {
  return (
    <React.StrictMode>
      <QueryClientProvider client={queryClient}>
        <MainDisplay />
      </QueryClientProvider>
    </React.StrictMode>
  )
}

export default App
