"use client";

import { Inbox, Send, File, Trash, Archive, Plus, ChevronDown } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";

function cn(...inputs: any[]) {
  return twMerge(clsx(inputs));
}

const navItems = [
  { icon: Inbox, label: "Inbox", href: "/inbox", count: 12 },
  { icon: Send, label: "Sent", href: "/sent" },
  { icon: File, label: "Drafts", href: "/drafts", count: 2 },
  { icon: Trash, label: "Trash", href: "/trash" },
  { icon: Archive, label: "Archive", href: "/archive" },
];

export default function Sidebar({ onCompose }: { onCompose: () => void }) {
  const pathname = usePathname();

  return (
    <aside className="w-64 flex flex-col h-[calc(100vh-64px)] bg-white py-4 overflow-y-auto">
      <div className="px-4 mb-4">
        <button 
          onClick={onCompose}
          className="flex items-center gap-3 bg-blue-100 hover:bg-blue-200 text-blue-900 px-6 py-4 rounded-2xl shadow-sm transition-all font-medium"
        >
          <Plus className="w-6 h-6" />
          <span>Compose</span>
        </button>
      </div>

      <nav className="flex-1">
        <ul className="space-y-0.5 px-2">
          {navItems.map((item) => (
            <li key={item.label}>
              <Link
                href={item.href}
                className={cn(
                  "flex items-center justify-between px-4 py-1.5 rounded-r-full text-sm font-medium transition-colors",
                  pathname === item.href
                    ? "bg-blue-100 text-blue-700"
                    : "text-gray-600 hover:bg-gray-100"
                )}
              >
                <div className="flex items-center gap-4">
                  <item.icon className={cn("w-5 h-5", pathname === item.href ? "text-blue-700" : "text-gray-500")} />
                  <span>{item.label}</span>
                </div>
                {item.count && (
                  <span className={cn("text-xs font-semibold", pathname === item.href ? "text-blue-700" : "text-gray-500")}>
                    {item.count}
                  </span>
                )}
              </Link>
            </li>
          ))}
        </ul>

        <div className="mt-8 px-4">
          <div className="flex items-center justify-between text-gray-700 font-medium mb-2 group cursor-pointer">
            <span>Labels</span>
            <Plus className="w-4 h-4 opacity-0 group-hover:opacity-100" />
          </div>
          <ul className="space-y-1">
            <li className="flex items-center gap-4 px-2 py-1 text-sm text-gray-600 hover:bg-gray-100 rounded-r-full cursor-pointer">
              <div className="w-3 h-3 rounded-full bg-red-500" />
              <span>Work</span>
            </li>
            <li className="flex items-center gap-4 px-2 py-1 text-sm text-gray-600 hover:bg-gray-100 rounded-r-full cursor-pointer">
              <div className="w-3 h-3 rounded-full bg-yellow-500" />
              <span>Personal</span>
            </li>
          </ul>
        </div>
      </nav>
    </aside>
  );
}
