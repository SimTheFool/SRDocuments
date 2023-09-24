import { getA4FormatFromWidth } from "@/utils/a4format";
import { Theme } from "@radix-ui/themes";
import "./A4Format.css";

type A4FormatProps = {
  children: React.ReactNode;
  border?: boolean;
};

export const A4Format = ({ children, border = false }: A4FormatProps) => {
  const a4 = getA4FormatFromWidth(2500);

  return (
    <Theme
      style={{
        width: `${a4.width}px`,
        height: `${a4.height}px`,
        border: border ? "2px solid var(--gray-10)" : "unset",
        boxSizing: border ? "content-box" : "border-box",
      }}
    >
      {children}
    </Theme>
  );
};
