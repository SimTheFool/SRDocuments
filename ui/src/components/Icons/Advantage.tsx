import { Box } from "@radix-ui/themes";
import { BsFillDiamondFill } from "react-icons/bs";
import { BaseIcon } from "./BaseIcon";

type AdvantageProps = {
  n: number;
};

export const Advantage = ({ n }: AdvantageProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        position: "relative",
        fontStyle: "normal",
      }}
    >
      <BaseIcon size={18}>
        <BsFillDiamondFill />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          fontWeight: "bold",
          color: "white",
          top: "13%",
          left: "20%",
          transform: "translate(-20%, 0%)",
          fontSize: "calc(13px * var(--scaling))",
        }}
      >
        {n >= 0 ? `+${n}` : n}
      </Box>
    </Box>
  );
};
