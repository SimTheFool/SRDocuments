import { getA4FormatFromWidth } from "@/utils/a4format";
import { Theme } from "@radix-ui/themes";
import "./A4Format.css";

type A4FormatProps = {
  children: React.ReactNode;
};

export const A4Format = ({ children }: A4FormatProps) => {
  const a4 = getA4FormatFromWidth(2500);

  return (
    <Theme
      style={{
        width: `${a4.width}px`,
        height: `${a4.height}px`,
      }}
    >
      {children}
    </Theme>
  );
};
