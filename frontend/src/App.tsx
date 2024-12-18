import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { useWeb3React } from "@web3-react/core";
import Card from "../components/Card";

function App() {
  const { connector, hooks } = useWeb3React();

  return (
    <>
    <div className="App">
      <div className="card">
        <Card connector={connector} hooks={hooks} name='phantom' />
      </div>
    </div>
    </>
  );
}

export default App;