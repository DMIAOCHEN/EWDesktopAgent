import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./SystemList.css";

interface System {
  id: string;
  name: string;
  url: string;
  icon?: string;
  enabled: boolean;
}

interface Props {
  onSelectSystem: (url: string) => void;
}

function SystemList({ onSelectSystem }: Props) {
  const [systems, setSystems] = useState<System[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadBusinessSystems();
  }, []);

  const loadBusinessSystems = async () => {
    try {
      const loadedSystems = await invoke<System[]>("load_business_systems");
      setSystems(loadedSystems.filter(s => s.enabled));
    } catch (e) {
      console.error("Failed to load business systems:", e);
      // Use defaults on error
      setSystems([
        { id: "ris", name: "RIS - 放射信息系统", url: "http://localhost:8080/ris", enabled: true },
        { id: "pis", name: "PIS - 病理信息系统", url: "http://localhost:8080/pis", enabled: true },
        { id: "eis", name: "EIS - 检验信息系统", url: "http://localhost:8080/eis", enabled: true },
      ]);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="system-list">
        <h3>业务系统</h3>
        <p>Loading...</p>
      </div>
    );
  }

  return (
    <div className="system-list">
      <h3>业务系统</h3>
      <ul>
        {systems.map((system) => (
          <li key={system.id} onClick={() => onSelectSystem(system.url)}>
            <span className="system-icon">{system.icon || "🏥"}</span>
            <span className="system-name">{system.name}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default SystemList;
