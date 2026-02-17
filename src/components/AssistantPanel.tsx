import { useState } from "react";
import "./AssistantPanel.css";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
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

  const handleSend = () => {
    if (!input.trim()) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: "user",
      content: input,
    };

    setMessages([...messages, userMessage]);
    setInput("");

    // TODO: Integrate with FastGPT API
    // Simulate AI response
    setTimeout(() => {
      const aiMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: "好的，我理解了您的问题。正在为您处理...",
      };
      setMessages((prev) => [...prev, aiMessage]);
    }, 1000);
  };

  return (
    <div className="assistant-panel">
      <div className="assistant-header">
        <h3>智能助手</h3>
      </div>

      <div className="assistant-messages">
        {messages.map((msg) => (
          <div key={msg.id} className={`message ${msg.role}`}>
            {msg.content}
          </div>
        ))}
      </div>

      <div className="assistant-input">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={(e) => e.key === "Enter" && handleSend()}
          placeholder="输入您的需求..."
        />
        <button onClick={handleSend}>发送</button>
      </div>
    </div>
  );
}

export default AssistantPanel;
