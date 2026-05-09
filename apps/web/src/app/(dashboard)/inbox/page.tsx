"use client";

import { useState, useEffect } from "react";
import { Star, Square, CheckSquare, MoreVertical, RefreshCcw, ChevronLeft, ChevronRight, Bell } from "lucide-react";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { useWebSocket } from "@/lib/use-websocket";

function cn(...inputs: any[]) {
  return twMerge(clsx(inputs));
}

export default function InboxPage() {
  const [messages, setMessages] = useState<any[]>([]);
  const [selectedMessages, setSelectedMessages] = useState<string[]>([]);
  const [notification, setNotification] = useState<string | null>(null);
  const { lastEvent } = useWebSocket();

  useEffect(() => {
    // Initial load (using mock data for now)
    setMessages([
      {
        id: "1",
        sender: "GitHub",
        subject: "[GitHub] A first-party personal access token was created",
        snippet: "Hey hemdai, A first-party personal access token (GitHub CLI) was recently...",
        date: "10:45 AM",
        isRead: false,
        isStarred: true,
      },
      {
        id: "2",
        sender: "Vercel",
        subject: "Deployment Successful: hemdaimail-web",
        snippet: "Your deployment for hemdaimail-web is now live. Preview: https://hemdaimail...",
        date: "9:30 AM",
        isRead: true,
        isStarred: false,
      },
    ]);
  }, []);

  useEffect(() => {
    if (lastEvent?.type === "NEW_MESSAGE") {
      const newMsg = lastEvent.payload;
      setMessages((prev) => [
        {
          id: newMsg.id,
          sender: newMsg.sender || "Unknown",
          subject: newMsg.subject || "(No Subject)",
          snippet: newMsg.snippet || "",
          date: "Just now",
          isRead: false,
          isStarred: false,
        },
        ...prev,
      ]);
      setNotification(`New message from ${newMsg.sender}`);
      setTimeout(() => setNotification(null), 5000);
    }
  }, [lastEvent]);

  const toggleSelect = (id: string) => {
    setSelectedMessages((prev) =>
      prev.includes(id) ? prev.filter((i) => i !== id) : [...prev, id]
    );
  };

  return (
    <div className="flex flex-col h-full relative">
      {/* Toast Notification */}
      {notification && (
        <div className="absolute top-4 left-1/2 -translate-x-1/2 z-50 bg-blue-600 text-white px-6 py-3 rounded-full shadow-2xl flex items-center gap-3 animate-bounce">
          <Bell className="w-5 h-5" />
          <span className="font-medium">{notification}</span>
        </div>
      )}

      {/* Inbox Toolbar */}
      <div className="flex items-center justify-between px-4 py-2 bg-white border-b border-gray-200">
        <div className="flex items-center gap-1">
          <button className="p-2 hover:bg-gray-100 rounded-lg">
            <Square className="w-5 h-5 text-gray-400" />
          </button>
          <button className="p-2 hover:bg-gray-100 rounded-lg">
            <RefreshCcw className="w-5 h-5 text-gray-500" />
          </button>
          <button className="p-2 hover:bg-gray-100 rounded-lg">
            <MoreVertical className="w-5 h-5 text-gray-500" />
          </button>
        </div>
        <div className="flex items-center gap-4 text-sm text-gray-500">
          <span>{messages.length} messages</span>
          <div className="flex items-center gap-1">
            <button className="p-2 hover:bg-gray-100 rounded-lg disabled:opacity-30">
              <ChevronLeft className="w-5 h-5" />
            </button>
            <button className="p-2 hover:bg-gray-100 rounded-lg">
              <ChevronRight className="w-5 h-5" />
            </button>
          </div>
        </div>
      </div>

      {/* Message List */}
      <div className="flex-1 overflow-auto">
        <table className="w-full border-collapse">
          <tbody>
            {messages.map((msg) => (
              <tr
                key={msg.id}
                className={cn(
                  "group border-b border-gray-100 cursor-pointer transition-all",
                  msg.isRead ? "bg-white" : "bg-gray-50 font-bold",
                  selectedMessages.includes(msg.id) ? "bg-blue-50" : "hover:shadow-md hover:z-10 relative"
                )}
              >
                <td className="pl-4 py-3 w-12">
                  <div className="flex items-center gap-2">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleSelect(msg.id);
                      }}
                      className="text-gray-300 hover:text-gray-500"
                    >
                      {selectedMessages.includes(msg.id) ? (
                        <CheckSquare className="w-5 h-5 text-blue-600" />
                      ) : (
                        <Square className="w-5 h-5" />
                      )}
                    </button>
                    <button
                      onClick={(e) => e.stopPropagation()}
                      className={cn(
                        "hover:text-yellow-500 transition-colors",
                        msg.isStarred ? "text-yellow-400" : "text-gray-300"
                      )}
                    >
                      <Star className={cn("w-5 h-5", msg.isStarred && "fill-current")} />
                    </button>
                  </div>
                </td>
                <td className="px-4 py-3 w-64 truncate text-sm">
                  {msg.sender}
                </td>
                <td className="px-4 py-3 truncate text-sm">
                  <span className="text-gray-900">{msg.subject}</span>
                  <span className="text-gray-500 font-normal"> — {msg.snippet}</span>
                </td>
                <td className="pr-4 py-3 w-24 text-right text-xs font-medium text-gray-500">
                  {msg.date}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
