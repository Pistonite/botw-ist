import './App.css'
import { CodeEditor } from '@pistonite/intwc'

function App() {

  return (
    <>
      <div style={{height: "500px"}}>
            <CodeEditor onCreated={async (api) => {
                    api.openFile("/main.ts", "const f = console.log; console.log('Hello, World!')", "typescript");
                }}/>
      </div>
    </>
  )
}

export default App
