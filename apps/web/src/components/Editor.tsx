"use client";

import { useEditor, EditorContent } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import Underline from "@tiptap/extension-underline";
import Link from "@tiptap/extension-link";
import Placeholder from "@tiptap/extension-placeholder";
import { Bold, Italic, Underline as UnderlineIcon, List, ListOrdered, Link as LinkIcon, Image as ImageIcon } from "lucide-react";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";

function cn(...inputs: any[]) {
  return twMerge(clsx(inputs));
}

export default function Editor({ content, onChange }: { content: string, onChange: (html: string) => void }) {
  const editor = useEditor({
    extensions: [
      StarterKit,
      Underline,
      Link.configure({ openOnClick: false }),
      Placeholder.configure({ placeholder: "Write your message..." }),
    ],
    content,
    onUpdate: ({ editor }) => {
      onChange(editor.getHTML());
    },
  });

  if (!editor) return null;

  return (
    <div className="flex flex-col border border-gray-200 rounded-lg overflow-hidden min-h-[300px]">
      <div className="flex items-center gap-1 p-2 bg-gray-50 border-b border-gray-200 flex-wrap">
        <button
          onClick={() => editor.chain().focus().toggleBold().run()}
          className={cn("p-1.5 rounded hover:bg-gray-200", editor.isActive("bold") && "bg-gray-200")}
        >
          <Bold className="w-4 h-4" />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleItalic().run()}
          className={cn("p-1.5 rounded hover:bg-gray-200", editor.isActive("italic") && "bg-gray-200")}
        >
          <Italic className="w-4 h-4" />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleUnderline().run()}
          className={cn("p-1.5 rounded hover:bg-gray-200", editor.isActive("underline") && "bg-gray-200")}
        >
          <UnderlineIcon className="w-4 h-4" />
        </button>
        <div className="w-[1px] h-4 bg-gray-300 mx-1" />
        <button
          onClick={() => editor.chain().focus().toggleBulletList().run()}
          className={cn("p-1.5 rounded hover:bg-gray-200", editor.isActive("bulletList") && "bg-gray-200")}
        >
          <List className="w-4 h-4" />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleOrderedList().run()}
          className={cn("p-1.5 rounded hover:bg-gray-200", editor.isActive("orderedList") && "bg-gray-200")}
        >
          <ListOrdered className="w-4 h-4" />
        </button>
      </div>
      <EditorContent editor={editor} className="flex-1 p-4 prose prose-sm max-w-none focus:outline-none" />
    </div>
  );
}
