import { Box, Text } from "@radix-ui/themes";
import { Space } from "../Space";
import { TextIndice } from "../TextIndice";

type SlotProps = {
  children?: React.ReactNode;
  size: "S" | "M" | "L" | "XL";
  concealment?: number;
};

const sizes = {
  S: "25px",
  M: "35px",
  L: "50px",
  XL: "65px",
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
          zIndex: 1,
          position: "absolute",
          bottom: "0",
          right: "0",
          lineHeight: "0.5",
          transform: "translate(0%, 2px)",
          backgroundColor: "white",
        }}
      >
        {children}

        {concealment && (
          <>
            <Space inline />
            <TextIndice>{concealment && `(d${concealment})`}</TextIndice>
          </>
        )}
      </Text>
    </Box>
  );
};
