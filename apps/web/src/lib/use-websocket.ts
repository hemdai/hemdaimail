import { useEffect, useRef, useState } from "react";
import { useAuth } from "./auth-context";

export function useWebSocket() {
  const { token } = useAuth();
  const [lastEvent, setLastEvent] = useState<any>(null);
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (!token) return;

    const wsUrl = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:4000/ws";
    // Axum doesn't natively support subprotocols for auth easily in the extractor,
    // but we can pass it as a query param or in the header if the browser allowed it.
    // For simplicity, we'll use query param for the JWT in the handshake.
    const socket = new WebSocket(`${wsUrl}?token=${token}`);

    socket.onopen = () => {
      console.log("WebSocket Connected");
    };

    socket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        setLastEvent(data);
      } catch (err) {
        console.error("Failed to parse WebSocket message", err);
      }
    };

    socket.onclose = () => {
      console.log("WebSocket Disconnected");
      // Optional: implement reconnect logic
    };

    socketRef.current = socket;

    return () => {
      socket.close();
    };
  }, [token]);

  return { lastEvent };
}
