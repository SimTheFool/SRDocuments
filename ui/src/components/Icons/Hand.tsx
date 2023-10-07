import { Box } from "@radix-ui/themes";
import { IoHandLeft } from "react-icons/io5";
import { BaseIcon } from "./BaseIcon";

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
          style={{
            color: "var(--gray-8)",
          }}
        />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          left: "40%",
          top: "58%",
          zIndex: 1,
          color: "white",
          transform: "translate(-50%, -50%)",
        }}
      >
        {n}
      </Box>
    </Box>
  );
};
