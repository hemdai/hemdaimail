"use client";

import { useState, useEffect } from "react";
import { Star, Square, CheckSquare, MoreVertical, RefreshCcw, ChevronLeft, ChevronRight, MailOpen, Send } from "lucide-react";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { api } from "@/lib/api";

function cn(...inputs: any[]) {
  return twMerge(clsx(inputs));
}

export default function SentPage() {
  const [messages, setMessages] = useState<any[]>([]);
  const [selectedMessages, setSelectedMessages] = useState<string[]>([]);
  const [selectedMessage, setSelectedMessage] = useState<any | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const fetchMessages = async () => {
    setIsLoading(true);
    try {
      const mailboxes = await api.mail.listMailboxes();
      const sent = mailboxes.find((m: any) => m.name.toLowerCase() === "sent");
      
      if (sent) {
        const data = await api.mail.listMessages(sent.id);
        setMessages(data);
      }
    } catch (err) {
      console.error("Failed to fetch sent messages", err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchMessages();
  }, []);

  const toggleSelect = (id: string) => {
    setSelectedMessages((prev) =>
      prev.includes(id) ? prev.filter((i) => i !== id) : [...prev, id]
    );
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    const now = new Date();
    if (date.toDateString() === now.toDateString()) {
      return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }
    return date.toLocaleDateString([], { month: 'short', day: 'numeric' });
  };

  if (selectedMessage) {
    return (
      <div className="flex flex-col h-full bg-white">
        <div className="flex items-center gap-4 p-4 border-b border-gray-100">
          <button onClick={() => setSelectedMessage(null)} className="p-2 hover:bg-gray-100 rounded-full">
            <ChevronLeft className="w-5 h-5" />
          </button>
          <h2 className="text-xl font-medium truncate">{selectedMessage.subject || "(No Subject)"}</h2>
        </div>
        <div className="flex-1 overflow-auto p-8">
          <div className="flex items-center justify-between mb-8">
            <div className="flex items-center gap-4">
              <div className="w-12 h-12 bg-gray-600 rounded-full flex items-center justify-center text-white font-bold text-xl">
                {selectedMessage.sender?.[0]?.toUpperCase()}
              </div>
              <div>
                <p className="font-bold text-gray-900">{selectedMessage.sender}</p>
                <p className="text-sm text-gray-500">to {selectedMessage.recipients?.join(", ")}</p>
              </div>
            </div>
            <p className="text-sm text-gray-500">{formatDate(selectedMessage.created_at)}</p>
          </div>
          <div className="prose prose-blue max-w-none text-gray-800 leading-relaxed whitespace-pre-wrap">
            {selectedMessage.body_text || selectedMessage.snippet}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-4 py-2 bg-white border-b border-gray-200">
        <div className="flex items-center gap-1">
          <button className="p-2 hover:bg-gray-100 rounded-lg">
            <Square className="w-5 h-5 text-gray-400" />
          </button>
          <button onClick={fetchMessages} className="p-2 hover:bg-gray-100 rounded-lg">
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

      <div className="flex-1 overflow-auto bg-white">
        {isLoading ? (
          <div className="flex items-center justify-center h-full">
            <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-blue-500"></div>
          </div>
        ) : messages.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-gray-400">
            <Send className="w-16 h-16 mb-4 opacity-20" />
            <p className="text-xl">No sent messages</p>
          </div>
        ) : (
          <table className="w-full border-collapse">
            <tbody>
              {messages.map((msg) => (
                <tr
                  key={msg.id}
                  onClick={() => setSelectedMessage(msg)}
                  className={cn(
                    "group border-b border-gray-100 cursor-pointer transition-all hover:shadow-sm bg-white",
                    selectedMessages.includes(msg.id) ? "bg-blue-50" : "hover:bg-gray-50"
                  )}
                >
                  <td className="pl-4 py-3 w-12 text-gray-900">
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
                    </div>
                  </td>
                  <td className="px-4 py-3 w-64 truncate text-sm text-gray-900">
                    To: {msg.recipients?.[0] || "Unknown"}
                  </td>
                  <td className="px-4 py-3 truncate text-sm">
                    <span className="text-gray-900">{msg.subject || "(No Subject)"}</span>
                    <span className="text-gray-500 font-normal"> — {msg.snippet}</span>
                  </td>
                  <td className="pr-4 py-3 w-24 text-right text-xs font-medium text-gray-500">
                    {formatDate(msg.created_at)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}
