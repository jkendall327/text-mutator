import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)
  const [backendOk, setBackendOk] = useState<string>("dead")

  const fetchUser = async () => {
      //const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';
      //const response = await fetch(`${apiUrl}/api/v1/health/`);
      const response = await fetch(`http://localhost:8080/api/v1/health`);

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      setBackendOk("live");
  };

  useEffect(() => {
    fetchUser();
  }, []);

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Backend: {backendOk}
      </p>
    </>
  )
}

export default App
