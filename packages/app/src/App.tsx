import './App.css'
import { CodeEditor } from '@pistonite/intwc'

function App() {

  return (
    <>
      <div style={{height: "500px"}}>
                <CodeEditor onCreated={async (api) => {
                    api.openFile("/main.tsx", "print('Hello, World!')", "typescript");
                }}/>
      </div>
    </>
  )
}

export default App
