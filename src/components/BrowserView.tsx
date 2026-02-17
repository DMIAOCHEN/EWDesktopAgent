import { useEffect, useRef, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./BrowserView.css";

interface BrowserTab {
  id: string;
  url: string;
  title: string;
  is_active: boolean;
}

interface Props {
  tabs: BrowserTab[];
  activeTabId: string | null;
  onTabSelect: (tabId: string) => void;
  onTabClose: (tabId: string) => void;
  onNewTab: (url: string) => void;
  currentUrl: string;
}

function BrowserView({ tabs, activeTabId, onTabSelect, onTabClose, onNewTab, currentUrl }: Props) {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [url, setUrl] = useState(currentUrl);
  const iframeRef = useRef<HTMLIFrameElement>(null);

  // Update URL when prop changes
  useEffect(() => {
    if (currentUrl && currentUrl !== url) {
      setUrl(currentUrl);
      setLoading(true);
      setError(null);
    }
  }, [currentUrl]);

  const handleNavigate = useCallback(async (newUrl: string) => {
    if (!newUrl) return;

    // Validate URL
    try {
      new URL(newUrl);
    } catch {
      // Add https:// if missing
      if (!newUrl.startsWith('http://') && !newUrl.startsWith('https://')) {
        newUrl = 'https://' + newUrl;
      }
    }

    setUrl(newUrl);
    setLoading(true);
    setError(null);

    // Navigate via Tauri command
    if (activeTabId) {
      try {
        await invoke("navigate_tab", { tabId: activeTabId, url: newUrl });
      } catch (e) {
        console.error("Navigation failed:", e);
      }
    }
  }, [activeTabId]);

  const handleBack = () => {
    if (iframeRef.current?.contentWindow) {
      iframeRef.current.contentWindow.history.back();
    }
  };

  const handleForward = () => {
    if (iframeRef.current?.contentWindow) {
      iframeRef.current.contentWindow.history.forward();
    }
  };

  const handleRefresh = () => {
    setLoading(true);
    if (iframeRef.current) {
      iframeRef.current.src = url;
    }
  };

  const handleLoad = () => {
    setLoading(false);
  };

  const handleError = () => {
    setLoading(false);
    setError("Failed to load page. Please check the URL.");
  };

  const handleUrlChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleNavigate(e.currentTarget.value);
    }
  };

  return (
    <div className="browser-view">
      {/* Tab bar */}
      <div className="browser-tabs">
        {tabs.map((tab) => (
          <div
            key={tab.id}
            className={`browser-tab ${tab.id === activeTabId ? "active" : ""}`}
            onClick={() => onTabSelect(tab.id)}
          >
            <span className="tab-title">{tab.title || tab.url}</span>
            <button
              className="tab-close"
              onClick={(e) => {
                e.stopPropagation();
                onTabClose(tab.id);
              }}
            >
              ×
            </button>
          </div>
        ))}
        <button className="new-tab-btn" onClick={() => onNewTab("")}>
          +
        </button>
      </div>

      {/* Toolbar */}
      <div className="browser-toolbar">
        <button onClick={handleBack} title="Back">←</button>
        <button onClick={handleForward} title="Forward">→</button>
        <button onClick={handleRefresh} title="Refresh">↻</button>
        <input
          type="text"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          onKeyDown={handleUrlChange}
          className="url-bar"
          placeholder="Enter URL..."
        />
      </div>

      {/* Content */}
      <div className="browser-content">
        {loading && (
          <div className="browser-loading">
            <div className="spinner"></div>
            <p>Loading...</p>
          </div>
        )}
        {error && (
          <div className="browser-error">
            <p>{error}</p>
            <button onClick={handleRefresh}>Retry</button>
          </div>
        )}
        {!loading && !error && (
          <iframe
            ref={iframeRef}
            src={url}
            title="Business System"
            sandbox="allow-same-origin allow-scripts allow-forms allow-popups"
            onLoad={handleLoad}
            onError={handleError}
          />
        )}
      </div>
    </div>
  );
}

export default BrowserView;
