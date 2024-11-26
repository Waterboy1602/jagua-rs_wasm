import './style/index.css';

function App() {
  return (
    <div className="App">
          <h1>Jagua-rs</h1>
          <form action="/json" method="post">
              <label for="json">JSON</label><br/>
              <textarea id="json_str" name="json_str" placeholder="Enter your JSON here..."></textarea>
              <br/><br/>
              <button type="submit">Submit</button>
          </form>
    </div>
  );
}

export default App;
