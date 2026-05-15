"use client";

import { Inbox, Send, File, Trash, Archive, Plus, ChevronDown, Tag } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";

function cn(...inputs: any[]) {
  return twMerge(clsx(inputs));
}

const navItems = [
  { icon: Inbox, label: "Inbox", href: "/inbox" },
  { icon: Send, label: "Sent", href: "/sent" },
  { icon: File, label: "Drafts", href: "/drafts" },
  { icon: Trash, label: "Trash", href: "/trash" },
  { icon: Archive, label: "Archive", href: "/archive" },
];

export default function Sidebar({ onCompose }: { onCompose: () => void }) {
  const pathname = usePathname();

  return (
    <aside className="w-64 flex flex-col h-[calc(100vh-64px)] bg-white py-4 overflow-y-auto border-r border-gray-100">
      <div className="px-4 mb-6">
        <button 
          onClick={onCompose}
          className="flex items-center gap-3 bg-blue-100 hover:bg-blue-200 text-blue-900 px-6 py-4 rounded-2xl shadow-sm transition-all font-semibold"
        >
          <Plus className="w-6 h-6" />
          <span>Compose</span>
        </button>
      </div>

      <nav className="flex-1">
        <ul className="space-y-0.5 pr-4">
          {navItems.map((item) => (
            <li key={item.label}>
              <Link
                href={item.href}
                className={cn(
                  "flex items-center justify-between px-4 py-2.5 rounded-r-full text-sm font-medium transition-all",
                  pathname === item.href
                    ? "bg-blue-50 text-blue-700 shadow-sm"
                    : "text-gray-600 hover:bg-gray-100"
                )}
              >
                <div className="flex items-center gap-4">
                  <item.icon className={cn("w-5 h-5", pathname === item.href ? "text-blue-700" : "text-gray-500")} />
                  <span>{item.label}</span>
                </div>
              </Link>
            </li>
          ))}
        </ul>

        <div className="mt-10 px-4">
          <div className="flex items-center justify-between text-gray-500 text-xs font-bold uppercase tracking-wider mb-4 px-2">
            <span>Labels</span>
            <Plus className="w-4 h-4 cursor-pointer hover:text-gray-900" />
          </div>
          <ul className="space-y-1">
            <li className="flex items-center gap-4 px-3 py-2 text-sm text-gray-600 hover:bg-gray-100 rounded-r-full cursor-pointer transition-colors">
              <div className="w-2.5 h-2.5 rounded-full bg-red-500" />
              <span>Work</span>
            </li>
            <li className="flex items-center gap-4 px-3 py-2 text-sm text-gray-600 hover:bg-gray-100 rounded-r-full cursor-pointer transition-colors">
              <div className="w-2.5 h-2.5 rounded-full bg-yellow-500" />
              <span>Personal</span>
            </li>
          </ul>
        </div>
      </nav>
    </aside>
  );
}
