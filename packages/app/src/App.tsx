import './App.css'
import { CodeEditor } from '@pistonite/intwc'

const CODE=`
function test() {
    let notconst = 1;
    const a = 1;
    const f = console.log;
    console.log('Hello, World!');

    type Foo = {
        readonly a: number;
    };

    const foo: Foo = { a: 1, bar: f };

    self.aIsReadonly = foo.a;
}
`

function App() {

  return (
    <>
      <div style={{height: "100%"}}>
            <CodeEditor onCreated={async (api) => {
                    api.openFile("/main.ts", CODE, "typescript");
                }}/>
      </div>
    </>
  )
}

export default App
