import { useState, useEffect, useRef } from "react";
import "./VoiceControl.css";

interface Props {
  onVoiceInput: (text: string) => void;
}

function VoiceControl({ onVoiceInput }: Props) {
  const [isListening, setIsListening] = useState(false);
  const [isSpeaking, setIsSpeaking] = useState(false);
  const [voiceState, setVoiceState] = useState<"idle" | "listening" | "processing" | "speaking">("idle");
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);

  const startListening = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const mediaRecorder = new MediaRecorder(stream);
      mediaRecorderRef.current = mediaRecorder;

      mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          // TODO: Send audio data to ASR service
          console.log("Audio data available:", event.data.size);
        }
      };

      mediaRecorder.start();
      setIsListening(true);
      setVoiceState("listening");
    } catch (error) {
      console.error("Failed to start voice recognition:", error);
    }
  };

  const stopListening = () => {
    if (mediaRecorderRef.current) {
      mediaRecorderRef.current.stop();
      mediaRecorderRef.current.stream.getTracks().forEach(track => track.stop());
    }
    setIsListening(false);
    setVoiceState("processing");

    // Simulate processing
    setTimeout(() => {
      setVoiceState("idle");
      // Would receive text from ASR in production
      onVoiceInput("模拟语音输入");
    }, 1500);
  };

  const speak = (text: string) => {
    if (!window.speechSynthesis) {
      console.error("Speech synthesis not supported");
      return;
    }

    const utterance = new SpeechSynthesisUtterance(text);
    utterance.lang = "zh-CN";
    utterance.onstart = () => setIsSpeaking(true);
    utterance.onend = () => setIsSpeaking(false);
    utterance.onerror = () => setIsSpeaking(false);

    window.speechSynthesis.speak(utterance);
    setVoiceState("speaking");
  };

  return (
    <div className="voice-control">
      <button
        className={`voice-btn ${voiceState}`}
        onClick={isListening ? stopListening : startListening}
        title={isListening ? "点击停止" : "点击说话"}
      >
        {voiceState === "idle" && "🎤"}
        {voiceState === "listening" && "👂"}
        {voiceState === "processing" && "⏳"}
        {voiceState === "speaking" && "🔊"}
      </button>

      <div className="voice-indicator">
        {voiceState === "listening" && <span className="listening-text">正在聆听...</span>}
        {voiceState === "processing" && <span className="processing-text">处理中...</span>}
        {voiceState === "speaking" && <span className="speaking-text">正在播报...</span>}
      </div>
    </div>
  );
}

export default VoiceControl;
