import { useState, useRef, useEffect } from "react";
import "./AssistantPanel.css";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
  actions?: AiAction[];
}

interface AiAction {
  action_type: string;
  target: string;
  value?: string;
}

function AssistantPanel() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: "1",
      role: "assistant",
      content: "您好！我是您的智能助手。可以帮您操作业务系统、查询信息等。请告诉我您需要什么帮助？",
    },
  ]);
  const [input, setInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);
  const [pendingAction, setPendingAction] = useState<AiAction | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: "user",
      content: input,
    };

    setMessages((prev) => [...prev, userMessage]);
    const currentInput = input;
    setInput("");
    setIsLoading(true);

    try {
      // Call AI API via Tauri
      // For now, simulate response - would call invoke("ai_chat", { messages })
      const response = await simulateAIResponse(currentInput);

      if (response.actions && response.actions.length > 0) {
        // Check if action requires confirmation
        const highRisk = response.actions.some(
          (a: AiAction) => a.action_type === "submit" || a.action_type === "delete"
        );

        if (highRisk) {
          setPendingAction(response.actions[0]);
          setShowConfirm(true);
        } else {
          // Execute low-risk actions directly
          executeActions(response.actions);
        }
      }

      const aiMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: response.content,
        actions: response.actions,
      };
      setMessages((prev) => [...prev, aiMessage]);
    } catch (e) {
      console.error("AI request failed:", e);
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: "抱歉，处理您的请求时出现错误。请稍后重试。",
      };
      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const simulateAIResponse = async (query: string) => {
    // Simulated AI response - in production, this would call the FastGPT API
    await new Promise((resolve) => setTimeout(resolve, 1000));

    const lowerQuery = query.toLowerCase();

    if (lowerQuery.includes("搜索") || lowerQuery.includes("查找")) {
      return {
        content: `好的，我来帮您搜索相关内容...`,
        actions: [{ action_type: "search", target: "current_page", value: query }],
      };
    }

    if (lowerQuery.includes("打开") || lowerQuery.includes("进入")) {
      return {
        content: `正在为您打开相关页面...`,
        actions: [{ action_type: "navigate", target: "new_page", value: query }],
      };
    }

    return {
      content: `我理解了，您说"${query}"。还有什么可以帮您的吗？`,
      actions: [],
    };
  };

  const executeActions = (actions: AiAction[]) => {
    // Execute browser actions via postMessage to iframe or Tauri commands
    console.log("Executing actions:", actions);
    // TODO: Implement actual action execution
  };

  const handleConfirm = () => {
    if (pendingAction) {
      executeActions([pendingAction]);
    }
    setShowConfirm(false);
    setPendingAction(null);
  };

  const handleCancel = () => {
    setShowConfirm(false);
    setPendingAction(null);

    // Add cancellation message
    const cancelMsg: Message = {
      id: Date.now().toString(),
      role: "assistant",
      content: "已取消操作。",
    };
    setMessages((prev) => [...prev, cancelMsg]);
  };

  return (
    <div className="assistant-panel">
      <div className="assistant-header">
        <h3>智能助手</h3>
      </div>

      <div className="assistant-messages">
        {messages.map((msg) => (
          <div key={msg.id} className={`message ${msg.role}`}>
            <div className="message-content">{msg.content}</div>
            {msg.actions && msg.actions.length > 0 && (
              <div className="message-actions">
                {msg.actions.map((action, idx) => (
                  <span key={idx} className="action-badge">
                    {action.action_type}
                  </span>
                ))}
              </div>
            )}
          </div>
        ))}
        {isLoading && (
          <div className="message assistant">
            <div className="typing-indicator">
              <span></span><span></span><span></span>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Risk confirmation dialog */}
      {showConfirm && (
        <div className="risk-confirm">
          <p>此操作可能存在风险，确定要执行吗？</p>
          <div className="confirm-buttons">
            <button className="confirm-btn" onClick={handleConfirm}>
              确认执行
            </button>
            <button className="cancel-btn" onClick={handleCancel}>
              取消
            </button>
          </div>
        </div>
      )}

      <div className="assistant-input">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSend()}
          placeholder="输入您的需求..."
          disabled={isLoading}
        />
        <button onClick={handleSend} disabled={isLoading || !input.trim()}>
          发送
        </button>
      </div>
    </div>
  );
}

export default AssistantPanel;
