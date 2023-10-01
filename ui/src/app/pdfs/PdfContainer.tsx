import { pdfsConfig } from "@/utils/config";
import { Theme } from "@radix-ui/themes";
import "./PdfContainer.css";

type A4FormatProps = {
  children: React.ReactNode;
  border?: boolean;
};

export const PdfContainer = ({ children, border = false }: A4FormatProps) => {
  return (
    <Theme
      style={{
        width: `${pdfsConfig.size.width}px`,
        height: `${pdfsConfig.size.height}px`,
        border: border ? "2px solid var(--gray-10)" : "unset",
        boxSizing: border ? "content-box" : "border-box",
      }}
    >
      {children}
    </Theme>
  );
};
