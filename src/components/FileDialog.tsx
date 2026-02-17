import { useState } from "react";
import "./FileDialog.css";

interface FileOperationDialogProps {
  isOpen: boolean;
  operation: "organize" | "move" | "copy" | "delete";
  onClose: () => void;
  onConfirm: (params: any) => void;
}

function FileDialog({ isOpen, operation, onClose, onConfirm }: FileOperationDialogProps) {
  const [sourceDir, setSourceDir] = useState("");
  const [targetDir, setTargetDir] = useState("");
  const [rule, setRule] = useState("by_date");
  const [preview, setPreview] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const handlePreview = async () => {
    if (!sourceDir) return;
    setIsLoading(true);
    // TODO: Call Tauri command to preview
    // const result = await invoke("preview_organization", { sourceDir, rule });
    setPreview([
      "Preview for: " + sourceDir,
      "Rule: " + rule,
      "---",
      "  file1.txt",
      "  file2.pdf",
      "  file3.jpg",
    ]);
    setIsLoading(false);
  };

  const handleConfirm = () => {
    onConfirm({ sourceDir, targetDir, rule });
    onClose();
  };

  if (!isOpen) return null;

  const getTitle = () => {
    switch (operation) {
      case "organize":
        return "整理文件";
      case "move":
        return "移动文件";
      case "copy":
        return "复制文件";
      case "delete":
        return "删除文件";
      default:
        return "文件操作";
    }
  };

  return (
    <div className="dialog-overlay">
      <div className="dialog-panel">
        <div className="dialog-header">
          <h3>{getTitle()}</h3>
          <button className="close-btn" onClick={onClose}>×</button>
        </div>

        <div className="dialog-content">
          {operation === "organize" && (
            <>
              <div className="form-group">
                <label>源目录</label>
                <input
                  type="text"
                  value={sourceDir}
                  onChange={(e) => setSourceDir(e.target.value)}
                  placeholder="选择要整理的文件夹"
                />
              </div>

              <div className="form-group">
                <label>整理规则</label>
                <select value={rule} onChange={(e) => setRule(e.target.value)}>
                  <option value="by_date">按日期</option>
                  <option value="by_type">按类型</option>
                  <option value="by_name">按名称</option>
                </select>
              </div>

              <button className="preview-btn" onClick={handlePreview} disabled={isLoading}>
                {isLoading ? "加载中..." : "预览"}
              </button>

              {preview.length > 0 && (
                <div className="preview-area">
                  <pre>{preview.join("\n")}</pre>
                </div>
              )}
            </>
          )}

          {operation === "move" && (
            <>
              <div className="form-group">
                <label>源文件</label>
                <input
                  type="text"
                  value={sourceDir}
                  onChange={(e) => setSourceDir(e.target.value)}
                  placeholder="源文件路径"
                />
              </div>

              <div className="form-group">
                <label>目标目录</label>
                <input
                  type="text"
                  value={targetDir}
                  onChange={(e) => setTargetDir(e.target.value)}
                  placeholder="目标目录"
                />
              </div>
            </>
          )}

          {operation === "delete" && (
            <div className="warning-message">
              <p>确定要删除以下文件吗？此操作不可撤销。</p>
              <input
                type="text"
                value={sourceDir}
                onChange={(e) => setSourceDir(e.target.value)}
                placeholder="输入要删除的文件路径"
              />
            </div>
          )}
        </div>

        <div className="dialog-footer">
          <button className="cancel-btn" onClick={onClose}>
            取消
          </button>
          <button className="confirm-btn" onClick={handleConfirm}>
            确认
          </button>
        </div>
      </div>
    </div>
  );
}

export default FileDialog;
