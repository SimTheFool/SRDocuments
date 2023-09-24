import { Box, Text } from "@radix-ui/themes";

type SlotProps = {
  children?: React.ReactNode;
  size: "S" | "M" | "L";
};

const sizes = {
  S: "100px",
  M: "200px",
  L: "400px",
} satisfies Record<SlotProps["size"], string>;

export const Slot = ({ children, size }: SlotProps) => {
  return (
    <Box
      style={{
        border: "2px dashed var(--gray-10)",
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
      </Text>
    </Box>
  );
};
