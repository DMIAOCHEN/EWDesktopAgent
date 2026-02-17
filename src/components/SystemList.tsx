import { useState } from "react";
import "./SystemList.css";

interface System {
  id: string;
  name: string;
  url: string;
  icon?: string;
}

const defaultSystems: System[] = [
  { id: "ris", name: "RIS - 放射信息系统", url: "http://localhost:3000/ris" },
  { id: "pis", name: "PIS - 病理信息系统", url: "http://localhost:3000/pis" },
  { id: "eis", name: "EIS - 检验信息系统", url: "http://localhost:3000/eis" },
];

interface Props {
  onSelectSystem: (url: string) => void;
}

function SystemList({ onSelectSystem }: Props) {
  const [systems] = useState<System[]>(defaultSystems);

  return (
    <div className="system-list">
      <h3>业务系统</h3>
      <ul>
        {systems.map((system) => (
          <li key={system.id} onClick={() => onSelectSystem(system.url)}>
            <span className="system-icon">{system.icon || "📋"}</span>
            <span className="system-name">{system.name}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default SystemList;
