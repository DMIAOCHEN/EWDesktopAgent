import { useEffect, useRef, useState } from "react";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import "./BrowserView.css";

interface Props {
  url: string;
}

function BrowserView({ url }: Props) {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    // Note: In Tauri 2.x, we'd use the webview API
    // For now, we'll show a placeholder
    const timer = setTimeout(() => setLoading(false), 1000);
    return () => clearTimeout(timer);
  }, [url]);

  if (loading) {
    return (
      <div className="browser-loading">
        <div className="spinner"></div>
        <p>Loading {url}...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="browser-error">
        <p>Error: {error}</p>
      </div>
    );
  }

  return (
    <div className="browser-view">
      <div className="browser-toolbar">
        <button>←</button>
        <button>→</button>
        <button>↻</button>
        <input type="text" value={url} readOnly className="url-bar" />
      </div>
      <div className="browser-content">
        <iframe
          src={url}
          title="Business System"
          sandbox="allow-same-origin allow-scripts allow-forms"
        />
      </div>
    </div>
  );
}

export default BrowserView;
