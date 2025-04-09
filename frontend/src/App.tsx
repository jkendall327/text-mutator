import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import './App.css'
import MainDisplay from './components/MainDisplay/MainDisplay'

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <MainDisplay />
    </QueryClientProvider>)
}

export default App
