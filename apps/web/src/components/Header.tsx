"use client";

import { Menu, Search, HelpCircle, Settings, Grid, User } from "lucide-react";
import { useAuth } from "@/lib/auth-context";

export default function Header() {
  const { user, logout } = useAuth();

  return (
    <header className="flex items-center justify-between px-4 py-2 bg-white border-b border-gray-200 h-16">
      <div className="flex items-center gap-4">
        <button className="p-2 hover:bg-gray-100 rounded-full">
          <Menu className="w-6 h-6 text-gray-600" />
        </button>
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 bg-blue-600 rounded flex items-center justify-center">
            <span className="text-white font-bold text-xl">H</span>
          </div>
          <span className="text-xl text-gray-700 font-medium">Hemdaimail</span>
        </div>
      </div>

      <div className="flex-1 max-w-2xl px-8">
        <div className="relative flex items-center">
          <div className="absolute left-3">
            <Search className="w-5 h-5 text-gray-500" />
          </div>
          <input
            type="text"
            placeholder="Search mail"
            className="w-full bg-gray-100 py-2.5 pl-12 pr-4 rounded-lg focus:bg-white focus:shadow-md outline-none border border-transparent focus:border-gray-200 transition-all"
          />
        </div>
      </div>

      <div className="flex items-center gap-2">
        <button className="p-2 hover:bg-gray-100 rounded-full">
          <HelpCircle className="w-6 h-6 text-gray-600" />
        </button>
        <button className="p-2 hover:bg-gray-100 rounded-full">
          <Settings className="w-6 h-6 text-gray-600" />
        </button>
        <button className="p-2 hover:bg-gray-100 rounded-full">
          <Grid className="w-6 h-6 text-gray-600" />
        </button>
        <div className="ml-2 relative group">
          <button className="w-10 h-10 bg-purple-600 rounded-full flex items-center justify-center text-white font-medium">
            {user?.email?.[0].toUpperCase() || "U"}
          </button>
          <div className="absolute right-0 mt-2 w-48 bg-white border border-gray-200 rounded-lg shadow-lg hidden group-hover:block z-50">
            <div className="p-4 border-b border-gray-100">
              <p className="text-sm font-medium text-gray-900 truncate">{user?.email}</p>
            </div>
            <button
              onClick={logout}
              className="w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-gray-50 rounded-b-lg"
            >
              Sign out
            </button>
          </div>
        </div>
      </div>
    </header>
  );
}
