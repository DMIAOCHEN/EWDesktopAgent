import { useState } from "react";
import "./Settings.css";

interface Settings {
  theme: "light" | "dark";
  language: string;
  voiceEnabled: boolean;
  autoStart: boolean;
  memoryLimit: number;
}

interface Props {
  onClose: () => void;
}

function Settings({ onClose }: Props) {
  const [settings, setSettings] = useState<Settings>({
    theme: "light",
    language: "zh-CN",
    voiceEnabled: true,
    autoStart: false,
    memoryLimit: 500,
  });

  const handleSave = () => {
    // TODO: Save settings to backend
    console.log("Saving settings:", settings);
    onClose();
  };

  return (
    <div className="settings-overlay">
      <div className="settings-panel">
        <div className="settings-header">
          <h2>设置</h2>
          <button className="close-btn" onClick={onClose}>
            ×
          </button>
        </div>

        <div className="settings-content">
          <div className="settings-section">
            <h3>外观</h3>
            <div className="setting-item">
              <label>主题</label>
              <select
                value={settings.theme}
                onChange={(e) =>
                  setSettings({ ...settings, theme: e.target.value as "light" | "dark" })
                }
              >
                <option value="light">浅色</option>
                <option value="dark">深色</option>
              </select>
            </div>
            <div className="setting-item">
              <label>语言</label>
              <select
                value={settings.language}
                onChange={(e) =>
                  setSettings({ ...settings, language: e.target.value })
                }
              >
                <option value="zh-CN">简体中文</option>
                <option value="en-US">English</option>
              </select>
            </div>
          </div>

          <div className="settings-section">
            <h3>语音</h3>
            <div className="setting-item">
              <label>启用语音交互</label>
              <input
                type="checkbox"
                checked={settings.voiceEnabled}
                onChange={(e) =>
                  setSettings({ ...settings, voiceEnabled: e.target.checked })
                }
              />
            </div>
          </div>

          <div className="settings-section">
            <h3>系统</h3>
            <div className="setting-item">
              <label>开机自启动</label>
              <input
                type="checkbox"
                checked={settings.autoStart}
                onChange={(e) =>
                  setSettings({ ...settings, autoStart: e.target.checked })
                }
              />
            </div>
            <div className="setting-item">
              <label>内存限制 (MB)</label>
              <input
                type="number"
                min="200"
                max="2000"
                value={settings.memoryLimit}
                onChange={(e) =>
                  setSettings({
                    ...settings,
                    memoryLimit: parseInt(e.target.value) || 500,
                  })
                }
              />
            </div>
          </div>

          <div className="settings-section">
            <h3>安全</h3>
            <div className="setting-item">
              <label>风险控制白名单</label>
              <button className="manage-btn">管理白名单</button>
            </div>
          </div>
        </div>

        <div className="settings-footer">
          <button className="cancel-btn" onClick={onClose}>
            取消
          </button>
          <button className="save-btn" onClick={handleSave}>
            保存
          </button>
        </div>
      </div>
    </div>
  );
}

export default Settings;
