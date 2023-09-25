import { Box, Text } from "@radix-ui/themes";
import { Space } from "./Space";

type SlotProps = {
  children?: React.ReactNode;
  size: "S" | "M" | "L" | "XL";
  concealment?: number;
};

const sizes = {
  S: "100px",
  M: "200px",
  L: "400px",
  XL: "800px",
} satisfies Record<SlotProps["size"], string>;

export const Slot = ({ children, size, concealment }: SlotProps) => {
  return (
    <Box
      style={{
        border: "1px dashed var(--gray-10)",
        height: sizes[size],
        position: "relative",
      }}
    >
      <Text
        size={"1"}
        style={{
          position: "absolute",
          bottom: "0",
          right: "0",
          lineHeight: "0.5",
        }}
      >
        {children}

        {concealment && (
          <>
            <Space inline />
            `(d${concealment})`
          </>
        )}
      </Text>
    </Box>
  );
};
