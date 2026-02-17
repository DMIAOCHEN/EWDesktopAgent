import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import SystemList from "./components/SystemList";
import BrowserView from "./components/BrowserView";
import AssistantPanel from "./components/AssistantPanel";
import "./styles/App.css";

function App() {
  const [activeTab, setActiveTab] = useState<string | null>(null);
  const [showAssistant, setShowAssistant] = useState(false);

  return (
    <div className="app">
      <header className="app-header">
        <h1>EW Desktop Agent</h1>
        <button onClick={() => setShowAssistant(!showAssistant)}>
          {showAssistant ? "Hide Assistant" : "Show Assistant"}
        </button>
      </header>

      <div className="app-content">
        <aside className="sidebar">
          <SystemList onSelectSystem={(url) => setActiveTab(url)} />
        </aside>

        <main className="main-content">
          {activeTab ? (
            <BrowserView url={activeTab} />
          ) : (
            <div className="empty-state">
              <p>Select a business system to get started</p>
            </div>
          )}
        </main>

        {showAssistant && (
          <aside className="assistant-panel">
            <AssistantPanel />
          </aside>
        )}
      </div>
    </div>
  );
}

export default App;
