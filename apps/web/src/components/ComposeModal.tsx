"use client";

import { X, Maximize2, Minimize2, Paperclip, Trash2 } from "lucide-react";
import { useState } from "react";
import Editor from "./Editor";

export default function ComposeModal({ onClose }: { onClose: () => void }) {
  const [to, setTo] = useState("");
  const [subject, setSubject] = useState("");
  const [content, setContent] = useState("");

  const handleSend = () => {
    console.log("Sending email:", { to, subject, content });
    // TODO: Call API
    onClose();
  };

  return (
    <div className="fixed bottom-0 right-8 w-[600px] bg-white rounded-t-xl shadow-2xl border border-gray-200 z-[100] flex flex-col overflow-hidden animate-in slide-in-from-bottom duration-300">
      <div className="flex items-center justify-between px-4 py-3 bg-gray-900 text-white cursor-pointer">
        <span className="text-sm font-medium">New Message</span>
        <div className="flex items-center gap-3">
          <Minimize2 className="w-4 h-4 text-gray-400 hover:text-white" />
          <Maximize2 className="w-4 h-4 text-gray-400 hover:text-white" />
          <X onClick={onClose} className="w-4 h-4 text-gray-400 hover:text-white" />
        </div>
      </div>

      <div className="flex flex-col flex-1 overflow-auto bg-white">
        <div className="px-4 py-2 border-b border-gray-100 flex items-center">
          <span className="text-sm text-gray-500 w-8">To</span>
          <input
            type="text"
            value={to}
            onChange={(e) => setTo(e.target.value)}
            className="flex-1 text-sm outline-none py-1"
          />
        </div>
        <div className="px-4 py-2 border-b border-gray-100 flex items-center">
          <span className="text-sm text-gray-500 w-8">Sub</span>
          <input
            type="text"
            value={subject}
            onChange={(e) => setSubject(e.target.value)}
            className="flex-1 text-sm outline-none py-1"
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
            className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-full transition-all"
          >
            Send
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
