import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import SystemList from "./components/SystemList";
import BrowserView from "./components/BrowserView";
import AssistantPanel from "./components/AssistantPanel";
import "./styles/App.css";

interface BrowserTab {
  id: string;
  url: string;
  title: string;
  is_active: boolean;
}

function App() {
  const [tabs, setTabs] = useState<BrowserTab[]>([]);
  const [activeTabId, setActiveTabId] = useState<string | null>(null);
  const [showAssistant, setShowAssistant] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  // Load tabs from backend on mount
  useEffect(() => {
    loadTabs();
  }, []);

  const loadTabs = async () => {
    try {
      const loadedTabs = await invoke<BrowserTab[]>("get_tabs");
      setTabs(loadedTabs);
      if (loadedTabs.length > 0) {
        const activeTab = loadedTabs.find(t => t.is_active);
        if (activeTab) {
          setActiveTabId(activeTab.id);
        }
      }
    } catch (e) {
      console.error("Failed to load tabs:", e);
    }
  };

  const handleSelectSystem = useCallback(async (url: string) => {
    setIsLoading(true);
    try {
      const newTab = await invoke<BrowserTab>("create_browser_tab", { url });
      setTabs(prev => [...prev, newTab]);
      setActiveTabId(newTab.id);
    } catch (e) {
      console.error("Failed to create tab:", e);
    } finally {
      setIsLoading(false);
    }
  }, []);

  const handleTabSelect = useCallback(async (tabId: string) => {
    try {
      await invoke("set_active_tab", { tabId });
      setActiveTabId(tabId);
      setTabs(prev => prev.map(t => ({
        ...t,
        is_active: t.id === tabId
      })));
    } catch (e) {
      console.error("Failed to set active tab:", e);
    }
  }, []);

  const handleTabClose = useCallback(async (tabId: string) => {
    try {
      await invoke("close_browser_tab", { tabId });
      setTabs(prev => prev.filter(t => t.id !== tabId));

      // If we closed the active tab, select another
      if (activeTabId === tabId) {
        const remaining = tabs.filter(t => t.id !== tabId);
        if (remaining.length > 0) {
          setActiveTabId(remaining[0].id);
        } else {
          setActiveTabId(null);
        }
      }
    } catch (e) {
      console.error("Failed to close tab:", e);
    }
  }, [activeTabId, tabs]);

  const handleNewTab = useCallback(async (url: string) => {
    const defaultUrl = url || "about:blank";
    try {
      const newTab = await invoke<BrowserTab>("create_browser_tab", { url: defaultUrl });
      setTabs(prev => [...prev, newTab]);
      setActiveTabId(newTab.id);
    } catch (e) {
      console.error("Failed to create new tab:", e);
    }
  }, []);

  // Get current URL from active tab
  const currentUrl = tabs.find(t => t.id === activeTabId)?.url || "";

  return (
    <div className="app">
      <header className="app-header">
        <h1>EW Desktop Agent</h1>
        <div className="header-actions">
          <button onClick={() => handleNewTab("")}>+ New Tab</button>
          <button onClick={() => setShowAssistant(!showAssistant)}>
            {showAssistant ? "Hide Assistant" : "Show Assistant"}
          </button>
        </div>
      </header>

      <div className="app-content">
        <aside className="sidebar">
          <SystemList onSelectSystem={handleSelectSystem} />
        </aside>

        <main className="main-content">
          {isLoading || tabs.length > 0 ? (
            <BrowserView
              tabs={tabs}
              activeTabId={activeTabId}
              onTabSelect={handleTabSelect}
              onTabClose={handleTabClose}
              onNewTab={handleNewTab}
              currentUrl={currentUrl}
            />
          ) : (
            <div className="empty-state">
              <p>Select a business system from the sidebar to get started</p>
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
