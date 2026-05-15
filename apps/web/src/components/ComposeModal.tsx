"use client";

import { X, Maximize2, Minimize2, Paperclip, Trash2 } from "lucide-react";
import { useState } from "react";
import Editor from "./Editor";
import { api } from "@/lib/api";

export default function ComposeModal({ onClose }: { onClose: () => void }) {
  const [to, setTo] = useState("");
  const [subject, setSubject] = useState("");
  const [content, setContent] = useState("");
  const [isSending, setIsSending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSend = async () => {
    if (!to || !subject) {
      setError("Please fill in recipient and subject");
      return;
    }

    setIsSending(true);
    setError(null);
    try {
      await api.mail.sendEmail({
        to: [to],
        subject,
        body: content,
      });
      onClose();
    } catch (err: any) {
      setError(err.message || "Failed to send email");
    } finally {
      setIsSending(false);
    }
  };

  return (
    <div className="fixed bottom-0 right-8 w-[600px] bg-white rounded-t-xl shadow-2xl border border-gray-200 z-[100] flex flex-col overflow-hidden animate-in slide-in-from-bottom duration-300">
      <div className="flex items-center justify-between px-4 py-3 bg-gray-900 text-white cursor-pointer">
        <span className="text-sm font-medium">New Message</span>
        <div className="flex items-center gap-3">
          <Minimize2 className="w-4 h-4 text-gray-400 hover:text-white" />
          <Maximize2 className="w-4 h-4 text-gray-400 hover:text-white" />
          <X onClick={onClose} className="w-4 h-4 text-gray-400 hover:text-white cursor-pointer" />
        </div>
      </div>

      <div className="flex flex-col flex-1 overflow-auto bg-white min-h-[400px]">
        {error && (
          <div className="px-4 py-2 bg-red-50 text-red-600 text-xs font-medium border-b border-red-100">
            {error}
          </div>
        )}
        <div className="px-4 py-2 border-b border-gray-100 flex items-center">
          <span className="text-sm text-gray-500 w-8">To</span>
          <input
            type="text"
            value={to}
            onChange={(e) => setTo(e.target.value)}
            className="flex-1 text-sm outline-none py-1 text-gray-900"
            placeholder="recipient@example.com"
          />
        </div>
        <div className="px-4 py-2 border-b border-gray-100 flex items-center">
          <span className="text-sm text-gray-500 w-8">Sub</span>
          <input
            type="text"
            value={subject}
            onChange={(e) => setSubject(e.target.value)}
            className="flex-1 text-sm outline-none py-1 text-gray-900"
            placeholder="Subject"
          />
        </div>

        <div className="flex-1 p-2">
          <Editor content={content} onChange={setContent} />
        </div>
      </div>

      <div className="px-4 py-3 bg-white border-t border-gray-100 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <button
            onClick={handleSend}
            disabled={isSending}
            className="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white font-bold py-2 px-6 rounded-full transition-all"
          >
            {isSending ? "Sending..." : "Send"}
          </button>
          <button className="p-2 hover:bg-gray-100 rounded-full">
            <Paperclip className="w-5 h-5 text-gray-600" />
          </button>
        </div>
        <button onClick={onClose} className="p-2 hover:bg-gray-100 rounded-full">
          <Trash2 className="w-5 h-5 text-gray-500" />
        </button>
      </div>
    </div>
  );
}
