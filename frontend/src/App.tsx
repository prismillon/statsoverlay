import "./App.css";
import UserDataDisplay from "./display";

function App() {
  const params = new URLSearchParams(window.location.search);
  const name = params.get("name");

  if (!name) {
    return (
      <div className="p-8">
        <p>Please provide a name parameter (e.g., /?name=john)</p>
      </div>
    );
  }

  return (
    <main className="p-8">
      <UserDataDisplay name={name} />
    </main>
  );
}

export default App;
