import "./App.css";

export const App = () => {
  return (
    <main className="app-shell">
      <section className="status-card" aria-labelledby="app-title">
        <p className="eyebrow">Project bootstrap</p>
        <h1 id="app-title">Open Claude Code</h1>
        <p className="status-copy">
          The desktop shell is ready. Provider setup and proxy controls come
          next.
        </p>
      </section>
    </main>
  );
};
