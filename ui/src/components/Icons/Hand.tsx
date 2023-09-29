import { BaseIcon } from "./BaseIcon";
import { PiHandThin } from "react-icons/pi";
import { IoHandLeft } from "react-icons/io5";
import { Box } from "@radix-ui/themes";

type HandProps = {
  n?: number;
};

export const Hand = ({ n }: HandProps) => {
  return (
    <Box
      style={{
        display: "block",
        position: "relative",
      }}
    >
      <BaseIcon size={25}>
        <IoHandLeft
          size="18"
          style={{
            color: "var(--gray-7)",
          }}
        />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          fontWeight: "bold",
          color: "var(--gray-10)",
          top: "0%",
          left: "20%",
        }}
      >
        {n}
      </Box>
    </Box>
  );
};
